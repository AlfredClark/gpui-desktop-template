; GPUI desktop template installer (unsigned local build).
; Replace names, publisher metadata and icon before shipping.
; Paths default to a repo-root-relative layout when ISCC runs from anywhere
; (relative to this script); CI overrides them with absolute paths via /D.
#define MyAppName "GpuiDesktop"
#ifndef MyAppVersion
#define MyAppVersion "0.0.0"
#endif
#ifndef MyAppSrc
#define MyAppSrc "..\\dist\\windows"
#endif
#ifndef MyAppOut
#define MyAppOut "..\\dist"
#endif

[Setup]
AppId=GpuiDesktopTemplate
AppName={#MyAppName}
AppVersion={#MyAppVersion}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
OutputDir={#MyAppOut}
OutputBaseFilename=GpuiDesktop-Setup
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

[Files]
Source: "{#MyAppSrc}\\app.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\app.exe"

[Run]
Filename: "{app}\app.exe"; Description: "Launch {#MyAppName}"; Flags: nowait postinstall skipifsilent
