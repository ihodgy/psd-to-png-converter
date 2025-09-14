# PSD to PNG Converter Run Script
Write-Host "Running PSD to PNG Converter..." -ForegroundColor Cyan
Write-Host ""

# Check if the release build exists
if (-not (Test-Path "target\release\psd-to-png-converter.exe")) {
    Write-Host "Release build not found. Building first..." -ForegroundColor Yellow
    & .\build.ps1
    if ($LASTEXITCODE -ne 0) {
        Read-Host "Press Enter to exit"
        exit 1
    }
}

Write-Host "Starting application..." -ForegroundColor Green
cargo run --release

Read-Host "Press Enter to exit"
