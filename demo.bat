@echo off
echo 🎨 PSD to PNG Converter Demo
echo ==========================
echo.

REM Add Rust to PATH
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

echo Building the application...
call build.bat

if %errorlevel% neq 0 (
    echo.
    echo ❌ Build failed! Please check the errors above.
    pause
    exit /b 1
)

echo.
echo ✅ Build successful!
echo.
echo 🚀 Starting the application...
echo.
echo Instructions:
echo 1. Drag a folder containing PSD files onto the application
echo 2. Select an output folder for the converted PNG files
echo 3. Click "Start Conversion" to begin the batch process
echo 4. Watch the progress bar and status updates
echo.
echo Example test folders:
echo - Input: examples\input\
echo - Output: examples\output\
echo.
echo Press any key to launch the application...
pause

cargo run --release

echo.
echo Demo completed!
pause
