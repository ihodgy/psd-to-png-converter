# PSD to PNG Converter Demo Script
Write-Host "🎨 PSD to PNG Converter Demo" -ForegroundColor Cyan
Write-Host "==========================" -ForegroundColor Cyan
Write-Host ""

Write-Host "Building the application..." -ForegroundColor Yellow
& .\build.ps1

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Build failed! Please check the errors above." -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-Host "✅ Build successful!" -ForegroundColor Green
Write-Host ""
Write-Host "🚀 Starting the application..." -ForegroundColor Cyan
Write-Host ""
Write-Host "Instructions:" -ForegroundColor White
Write-Host "1. Drag a folder containing PSD files onto the application" -ForegroundColor Gray
Write-Host "2. Select an output folder for the converted PNG files" -ForegroundColor Gray
Write-Host "3. Click 'Start Conversion' to begin the batch process" -ForegroundColor Gray
Write-Host "4. Watch the progress bar and status updates" -ForegroundColor Gray
Write-Host ""
Write-Host "Example test folders:" -ForegroundColor White
Write-Host "- Input: examples\input\" -ForegroundColor Gray
Write-Host "- Output: examples\output\" -ForegroundColor Gray
Write-Host ""
Read-Host "Press Enter to launch the application"

cargo run --release

Write-Host ""
Write-Host "Demo completed!" -ForegroundColor Green
Read-Host "Press Enter to exit"
