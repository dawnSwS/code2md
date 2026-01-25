#![windows_subsystem = "windows"]

use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use walkdir::{DirEntry, WalkDir};

// --- 忽略配置 ---
fn get_ignore_dirs() -> &'static HashSet<&'static str> {
    static DIRS: OnceLock<HashSet<&'static str>> = OnceLock::new();
    DIRS.get_or_init(|| {
        HashSet::from([
            ".git", ".idea", ".vscode", ".vs", "__pycache__", "node_modules", 
            "venv", ".venv", "env", "dist", "build", "target", "out", 
            "bin", "obj", "debug", "release", 
            ".gradle", "captures", "gradle", ".DS_Store", "coverage", ".next", ".nuxt"
        ])
    })
}

fn get_ignore_filenames() -> &'static HashSet<&'static str> {
    static FILES: OnceLock<HashSet<&'static str>> = OnceLock::new();
    FILES.get_or_init(|| {
        HashSet::from([
            "gradlew", "gradlew.bat", "mvnw", "mvnw.cmd",
            "local.properties", "thumbs.db", "desktop.ini", 
            "package-lock.json", "yarn.lock", "pnpm-lock.yaml", "cargo.lock", "poetry.lock"
        ])
    })
}

fn get_ignore_extensions() -> &'static HashSet<&'static str> {
    static EXTS: OnceLock<HashSet<&'static str>> = OnceLock::new();
    EXTS.get_or_init(|| {
        HashSet::from([
            // 媒体文件
            ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".svg", ".webp", ".tiff",
            ".mp3", ".mp4", ".wav", ".avi", ".mov",
            // 二进制/压缩包
            ".exe", ".dll", ".so", ".dylib", ".bin", ".apk", ".aab", ".jar", ".war",
            ".zip", ".tar", ".gz", ".7z", ".rar", ".iso", ".cab",
            // 编译中间产物
            ".pyc", ".class", ".o", ".obj", ".pdb", ".suo",
            ".db", ".sqlite", ".sqlite3", ".lock", ".log",
            // 新增：忽略 md 文件，避免递归处理或包含说明文档
            ".md"
        ])
    })
}

struct Args {
    path: String,
    save_inside: bool,
}

fn parse_args() -> Option<Args> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return None;
    }

    let path = args[1].clone();
    let save_inside = args.iter().any(|arg| arg == "-i");

    Some(Args { path, save_inside })
}

fn is_hidden_or_ignored(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_str().unwrap_or("");
    
    if entry.file_type().is_dir() {
        if file_name.starts_with('.') && file_name.len() > 1 && file_name != ".github" {
            return true;
        }
        if get_ignore_dirs().contains(file_name) { return true; }
    } else {
        if get_ignore_filenames().contains(&file_name.to_lowercase().as_str()) { return true; }
    }
    false
}

fn is_text_file(path: &Path) -> bool {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    
    let mut buffer = [0; 1024]; 
    let n = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(_) => return false,
    };
    if n == 0 { return true; }

    !buffer[..n].contains(&0)
}

fn run_app() -> io::Result<()> {
    let args = match parse_args() {
        Some(a) => a,
        None => return Ok(()),
    };

    let source_path = Path::new(&args.path).canonicalize()?;
    
    let name_os = source_path.file_name().unwrap_or(std::ffi::OsStr::new("项目代码文档"));
    let folder_name = name_os.to_string_lossy();
    
    // 修改：扩展名改为 .md
    let file_name = format!("{}.md", folder_name);

    let output_path = if source_path.is_dir() {
        if args.save_inside {
            source_path.join(file_name)
        } else {
            source_path.parent().unwrap_or(&source_path).join(file_name)
        }
    } else {
        source_path.parent().unwrap_or(&source_path).join(file_name)
    };

    let file = File::create(&output_path)?;
    let mut writer = BufWriter::new(file);

    let out_file_name_os = output_path.file_name().unwrap_or_default();
    let out_file_abs = output_path.canonicalize().unwrap_or_else(|_| output_path.clone());

    let walker = WalkDir::new(&source_path).into_iter();

    for entry in walker.filter_entry(|e| !is_hidden_or_ignored(e)) {
        let entry = match entry { Ok(e) => e, Err(_) => continue };
        let path = entry.path();

        if path.is_dir() { continue; }

        if path.file_name() == Some(out_file_name_os) { continue; }
        if let Ok(abs) = path.canonicalize() {
             if abs == out_file_abs { continue; }
        }

        if let Some(ext) = path.extension() {
            let ext_str = format!(".{}", ext.to_str().unwrap_or("").to_lowercase());
            if get_ignore_extensions().contains(ext_str.as_str()) { continue; }
        }

        if let Ok(meta) = path.metadata() {
            if meta.len() > 1024 * 1024 { continue; }
        }

        if !is_text_file(path) { continue; }

        match fs::read(path) {
            Ok(bytes) => {
                let content = String::from_utf8_lossy(&bytes);
                if content.trim().is_empty() { continue; }

                let rel_path = path.strip_prefix(&source_path).unwrap_or(path);
                let path_str = rel_path.display().to_string().replace("\\", "/");
                
                // 获取不带点的扩展名用于 Markdown 代码块标识
                let file_ext = path.extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                // 修改：写入 Markdown 格式
                writeln!(writer, "## File: {}\n", path_str)?;
                writeln!(writer, "```{}", file_ext)?;
                writeln!(writer, "{}", content)?;
                writeln!(writer, "```\n")?;
            }
            Err(_) => continue,
        }
    }
    
    writer.flush()?;

    Ok(())
}

fn main() {
    if let Err(_) = run_app() {
        std::process::exit(1);
    }
}