$ErrorActionPreference = 'Stop'
$bundle = (Resolve-Path 'dist/keycast-bridge-windows-x64').Path
$versionInfo = (Get-Item "$bundle\keycast-bridge.exe").VersionInfo
if ($versionInfo.ProductName -ne 'Keycast Bridge') { throw 'Missing PE product name' }
if ($versionInfo.FileVersion -ne $env:KEYCAST_VERSION -or $versionInfo.ProductVersion -ne $env:KEYCAST_VERSION) {
    throw 'PE version does not match Cargo.toml'
}
if (-not (Test-Path "$bundle\keycast-bridge.ico")) { throw 'Missing shortcut icon' }
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class KeycastIconCheck {
    [DllImport("shell32.dll", CharSet = CharSet.Unicode)]
    public static extern uint ExtractIconEx(string file, int index, IntPtr[] large, IntPtr[] small, uint count);
}
'@
if ([KeycastIconCheck]::ExtractIconEx("$bundle\keycast-bridge.exe", -1, $null, $null, 0) -lt 1) {
    throw 'Executable has no embedded icon'
}
foreach ($name in @('Keycast_Bridge_Guide_Utilisateur_FR', 'Keycast_Bridge_User_Guide_EN')) {
    if (-not (Test-Path "$bundle\guide\pdf\$name.pdf")) { throw "Missing PDF guide: $name" }
}
foreach ($language in @('en', 'fr')) {
    if (-not (Test-Path "$bundle\guide\$language.html")) { throw "Missing HTML guide: $language" }
}
$originalPath = $env:PATH
$runtime = $env:KEYCAST_UCRT_ROOT
if (-not $runtime -or -not (Test-Path -LiteralPath $runtime)) { throw 'Missing build runtime path' }
$hiddenRuntime = "$runtime-keycast-hidden"
Move-Item -LiteralPath $runtime -Destination $hiddenRuntime
try {
    # Runtime test must not accidentally use the MSYS2 installation.
    $env:PATH = "$env:SystemRoot\System32;$env:SystemRoot"
    $env:KEYCAST_SMOKE_TEST = '1'
    $app = Start-Process "$bundle\keycast-bridge.exe" -WorkingDirectory $bundle -PassThru
    if (-not $app.WaitForExit(20000)) { $app.Kill(); throw 'GUI smoke test timed out' }
    if ($app.ExitCode -ne 0) { throw "GUI failed: $($app.ExitCode)" }
} finally {
    Move-Item -LiteralPath $hiddenRuntime -Destination $runtime
    $env:PATH = $originalPath
    Remove-Item Env:KEYCAST_SMOKE_TEST -ErrorAction SilentlyContinue
}
Write-Host 'PASS: portable GTK GUI launched without MSYS2 on PATH.'
