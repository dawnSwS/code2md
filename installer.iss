#define MyAppName "Code2XML"
#define MyAppExeName "code2xml.exe"
#define MyAppPublisher "MyGitHubUser"

#ifndef MyAppVersion
  #define MyAppVersion "1.0.0"
#endif

[Setup]
AppId={{70605ddd-3851-47bb-bc68-f2cbcaecd10c}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\{#MyAppName}
DisableProgramGroupPage=yes
OutputBaseFilename=C2X_Setup
OutputDir=Output
Compression=lzma2/max
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=admin
ChangesAssociations=yes
SetupIconFile=app_icon.ico

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
; 修改：更新右键菜单文案为 "生成项目 Markdown"
Root: HKCR; Subkey: "Directory\shell\{#MyAppName}"; ValueType: string; ValueName: ""; ValueData: "生成项目 Markdown"; Flags: uninsdeletekey
Root: HKCR; Subkey: "Directory\shell\{#MyAppName}"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\{#MyAppExeName}"; Flags: uninsdeletekey
Root: HKCR; Subkey: "Directory\shell\{#MyAppName}\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Flags: uninsdeletekey

; 修改：更新背景右键菜单文案
Root: HKCR; Subkey: "Directory\Background\shell\{#MyAppName}"; ValueType: string; ValueName: ""; ValueData: "生成项目 Markdown (当前目录)"; Flags: uninsdeletekey
Root: HKCR; Subkey: "Directory\Background\shell\{#MyAppName}"; ValueType: string; ValueName: "Icon"; ValueData: "{app}\{#MyAppExeName}"; Flags: uninsdeletekey
Root: HKCR; Subkey: "Directory\Background\shell\{#MyAppName}\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%V"" -i"; Flags: uninsdeletekey