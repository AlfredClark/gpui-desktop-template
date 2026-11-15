; GPUI desktop template installer (unsigned local build).
; Replace names, publisher metadata and icon before shipping.
#define MyAppName "GpuiDesktop"
#ifndef MyAppVersion
#define MyAppVersion "0.0.0"
#endif

[Setup]
AppId=GpuiDesktopTemplate
AppName={#MyAppName}
AppVersion={#MyAppVersion}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
OutputDir=dist
OutputBaseFilename=GpuiDesktop-Setup
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

[Files]
Source: "dist\windows\app.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\app.exe"

[Run]
Filename: "{app}\app.exe"; Description: "Launch {#MyAppName}"; Flags: nowait postinstall skipifsilent
