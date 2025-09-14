# 🎨 PSD to PNG Converter

A beautiful, modern Rust application for batch converting Adobe Photoshop PSD files to PNG format with drag-and-drop functionality.

## ✨ Features

- **Drag & Drop Interface**: Simply drag your PSD folder onto the application
- **Batch Processing**: Convert multiple PSD files at once
- **Progress Tracking**: Real-time progress bar and status updates
- **Modern UI**: Beautiful, responsive interface built with egui
- **Error Handling**: Comprehensive error reporting for failed conversions
- **Cross-Platform**: Works on Windows, macOS, and Linux

## 🚀 Installation

### Prerequisites

1. **Install Rust**: Visit [rustup.rs](https://rustup.rs/) and follow the installation instructions
2. **Verify Installation**: Open a terminal and run:
   ```bash
   rustc --version
   cargo --version
   ```

### Building the Application

#### Method 1: Using Scripts (Recommended)
- **Windows Command Prompt**: Double-click `build.bat`
- **PowerShell**: Right-click `build.ps1` → "Run with PowerShell"
- **Demo**: Run `demo.bat` or `demo.ps1` for a guided experience

#### Method 2: Manual Commands
1. **Navigate to Project**: Open terminal in the project directory
2. **Build**: Run the following command:
   ```bash
   cargo build --release
   ```
3. **Run**: Execute the application:
   ```bash
   cargo run --release
   ```

## 📖 How to Use

### Quick Start
1. **Launch**: Run `cargo run --release` or use the demo scripts
2. **Input**: Drag your PSD folder onto the application window
3. **Output**: Select where you want the PNG files saved
4. **Convert**: Click "🚀 Start Conversion" and watch the progress
5. **Done**: Check your output folder for converted PNG files

### Detailed Instructions
- See [USAGE.md](USAGE.md) for comprehensive usage guide
- See [SETUP.md](SETUP.md) for detailed setup instructions
- Use `examples/input/` and `examples/output/` folders for testing

## 🛠️ Technical Details

### Dependencies

- **egui**: Modern, immediate mode GUI framework
- **image**: Image processing and format conversion
- **photoshop-psd**: PSD file parsing and extraction
- **tokio**: Async runtime for non-blocking operations
- **walkdir**: Recursive directory traversal
- **anyhow**: Error handling and propagation

### Architecture

- **Main App**: Handles application state and coordination
- **UI Module**: Manages the user interface and interactions
- **Converter Module**: Handles PSD to PNG conversion logic
- **Error Module**: Comprehensive error types and handling

## 🔧 Troubleshooting

### Common Issues

1. **"No PSD files found"**: Ensure your input folder contains `.psd` files
2. **Conversion fails**: Some PSD files may have complex features that aren't supported
3. **Permission errors**: Make sure you have write access to the output folder

### Supported PSD Features

- Basic layers and compositions
- RGB color mode
- Common layer effects
- Flattened image export

### Limitations

- Complex layer effects may not convert perfectly
- Some advanced PSD features are not supported
- Large files may take longer to process

## 🎨 Customization

The application uses a modern dark theme with customizable colors. You can modify the styling in `src/main.rs` in the `setup_custom_styles` function.

## 📝 License

This project is open source and available under the MIT License.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and enhancement requests.

## 📞 Support

If you encounter any issues or have questions, please create an issue in the project repository.

## 🎬 Demo & Examples

### Quick Demo
- **Windows**: Double-click `demo.bat` for a guided demo experience
- **PowerShell**: Run `.\demo.ps1` for a colored terminal demo
- **Manual**: Follow the instructions in [USAGE.md](USAGE.md)

### Test Setup
- **Input Folder**: `examples/input/` - Place your PSD files here
- **Output Folder**: `examples/output/` - Converted PNGs will appear here
- **Instructions**: See `examples/test-psd.txt` for detailed testing steps

### Project Structure
```
psd-to-png-converter/
├── src/                    # Source code
├── examples/               # Test folders and instructions
├── build.bat/.ps1         # Build scripts
├── run.bat/.ps1           # Run scripts  
├── demo.bat/.ps1          # Demo scripts
├── README.md              # This file
├── USAGE.md               # Detailed usage guide
└── SETUP.md               # Setup instructions
```

---

**Made with ❤️ in Rust**
