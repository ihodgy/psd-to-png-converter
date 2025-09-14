# Quick Setup Guide

## 🚀 Getting Started

### Step 1: Install Rust
1. Go to [https://rustup.rs/](https://rustup.rs/)
2. Download and run the installer
3. Follow the installation instructions
4. Restart your terminal/command prompt

### Step 2: Verify Installation
Open a new terminal and run:
```bash
rustc --version
cargo --version
```

Both commands should return version numbers.

### Step 3: Build and Run
Choose one of these methods:

#### Method 1: Using Scripts (Recommended)
- **Windows Command Prompt**: Double-click `build.bat`
- **PowerShell**: Right-click `build.ps1` → "Run with PowerShell"
- **To run**: Double-click `run.bat` or `run.ps1`

#### Method 2: Manual Commands
```bash
# Build the application
cargo build --release

# Run the application
cargo run --release
```

### Step 4: Using the Application
1. **Launch**: Run the application using one of the methods above
2. **Input**: Drag a folder containing PSD files onto the app
3. **Output**: Select where you want the PNG files saved
4. **Convert**: Click "Start Conversion" and watch the progress

## 🎯 Features

- ✨ **Beautiful UI**: Modern, dark theme interface
- 📁 **Drag & Drop**: Simply drag folders onto the app
- 🔄 **Batch Processing**: Convert multiple files at once
- 📊 **Progress Tracking**: Real-time progress updates
- 🛡️ **Error Handling**: Clear error messages for issues
- 🚀 **Fast**: Built with Rust for optimal performance

## 📁 Project Structure

```
psd-to-png-converter/
├── src/
│   ├── main.rs          # Main application logic
│   ├── ui.rs            # User interface
│   ├── converter.rs     # PSD to PNG conversion
│   └── error.rs         # Error handling
├── examples/
│   ├── input/           # Place PSD files here
│   └── output/          # Converted PNGs go here
├── Cargo.toml           # Project dependencies
├── README.md            # Detailed documentation
├── build.bat/.ps1       # Build scripts
└── run.bat/.ps1         # Run scripts
```

## 🔧 Troubleshooting

### "Rust not found" Error
- Make sure Rust is installed correctly
- Restart your terminal after installation
- Try running `rustup update` to ensure latest version

### Build Errors
- Check that you're in the project directory
- Try `cargo clean` then rebuild
- Make sure you have internet connection for downloading dependencies

### Conversion Issues
- Ensure PSD files are valid Adobe Photoshop files
- Check that you have write permissions to the output folder
- Some complex PSD features may not convert perfectly

## 💡 Tips

- Place PSD files in the `examples/input/` folder for testing
- The app maintains folder structure from input to output
- Large files may take time - be patient!
- Check the status messages for conversion progress

---

**Need help?** Check the main `README.md` for detailed documentation!
