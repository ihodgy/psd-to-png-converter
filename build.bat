@echo off
echo Building PSD to PNG Converter...
echo.

REM Add Rust to PATH
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

REM Check if Rust is installed
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Error: Rust is not installed or not in PATH
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

REM Build the application
echo Compiling...
cargo build --release

if %errorlevel% equ 0 (
    echo.
    echo ✅ Build successful!
    echo.
    echo You can now run the application with:
    echo   cargo run --release
    echo.
    echo Or find the executable at:
    echo   target\release\psd-to-png-converter.exe
    echo.
) else (
    echo.
    echo ❌ Build failed!
    echo Please check the error messages above.
    echo.
)

pause
