@echo off
echo Running PSD to PNG Converter...
echo.

REM Add Rust to PATH
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

REM Check if the release build exists
if not exist "target\release\psd-to-png-converter.exe" (
    echo Release build not found. Building first...
    call build.bat
    if %errorlevel% neq 0 (
        pause
        exit /b 1
    )
)

REM Run the application
echo Starting application...
cargo run --release

pause
