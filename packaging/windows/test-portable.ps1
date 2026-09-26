$ErrorActionPreference = 'Stop'
$bundle = (Resolve-Path 'dist/keycast-bridge-windows-x64').Path
$originalPath = $env:PATH
try {
    # Runtime test must not accidentally use the MSYS2 installation.
    $env:PATH = "$env:SystemRoot\System32;$env:SystemRoot"
    $env:KEYCAST_SMOKE_TEST = '1'
    $app = Start-Process "$bundle\keycast-bridge.exe" -WorkingDirectory $bundle -PassThru
    if (-not $app.WaitForExit(20000)) { $app.Kill(); throw 'GUI smoke test timed out' }
    if ($app.ExitCode -ne 0) { throw "GUI failed: $($app.ExitCode)" }
} finally {
    $env:PATH = $originalPath
    Remove-Item Env:KEYCAST_SMOKE_TEST -ErrorAction SilentlyContinue
}
Write-Host 'PASS: portable GTK GUI launched without MSYS2 on PATH.'
