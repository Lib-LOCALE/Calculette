# Build script for Windows 11 / 10 (x64)
param (
    [switch]$Run,
    [switch]$Debug
)

$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
Push-Location $projectRoot

try {
    Write-Host "==> Calculette Windows Build <==" -ForegroundColor Cyan

    # 1. Ensure Windows icon is generated
    if (-not (Test-Path "$projectRoot\icons\icon.ico")) {
        Write-Host "Generating icon.ico..." -ForegroundColor Yellow
        & "$PSScriptRoot\generate-icon.ps1"
    }

    # 2. Build with Cargo
    $buildArgs = @("build")
    if (-not $Debug) {
        $buildArgs += "--release"
    }

    Write-Host "Compiling with cargo $($buildArgs -join ' ')..." -ForegroundColor Yellow
    cargo @buildArgs

    $targetSubdir = if ($Debug) { "debug" } else { "release" }
    $exePath = "$projectRoot\target\$targetSubdir\calculatrice.exe"

    if (Test-Path $exePath) {
        $item = Get-Item $exePath
        Write-Host "Build complete: $($item.FullName) ($([math]::Round($item.Length / 1MB, 2)) MB)" -ForegroundColor Green
        if ($Run) {
            Write-Host "Launching $exePath..." -ForegroundColor Cyan
            & $exePath
        }
    } else {
        Write-Error "Executable not found at $exePath"
    }
}
finally {
    Pop-Location
}
