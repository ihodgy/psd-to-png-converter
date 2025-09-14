# 🎨 PSD to PNG Converter - Usage Guide

## 🚀 Quick Start

1. **Build the Application**:
   ```bash
   cargo build --release
   ```

2. **Run the Application**:
   ```bash
   cargo run --release
   ```
   Or use the provided scripts:
   - Windows: Double-click `build.bat` then `run.bat`
   - PowerShell: Run `.\build.ps1` then `.\run.ps1`

## 📖 How to Use

### Step 1: Launch the Application
- The application will open with a beautiful dark theme interface
- You'll see the title "🎨 PSD to PNG Converter" at the top
- A large drag-and-drop area in the center

### Step 2: Select Input Folder
**Option A: Drag & Drop (Recommended)**
- Simply drag a folder containing PSD files onto the application window
- The folder path will appear in the "Input Folder" section

**Option B: Browse Button**
- Click the "📂 Select Input Folder" button
- Navigate to your folder containing PSD files
- Click "Select Folder"

### Step 3: Select Output Folder
- Click the "📁 Select Output Folder" button
- Choose where you want the converted PNG files saved
- The application will preserve the folder structure

### Step 4: Start Conversion
- Once both folders are selected, the "🚀 Start Conversion" button becomes active
- Click it to begin the batch conversion process
- Watch the progress bar and status messages

### Step 5: Monitor Progress
- The progress bar shows conversion percentage
- Status messages provide real-time updates
- Any errors will be displayed in red

## ✨ Features

### 🎨 Beautiful Interface
- Modern dark theme with smooth animations
- Responsive design that adapts to window size
- Clear visual feedback for all interactions

### 📁 Drag & Drop Support
- Drag folders directly onto the application
- Visual feedback when hovering over drop zone
- Automatic folder detection

### 🔄 Batch Processing
- Convert multiple PSD files at once
- Preserves folder structure from input to output
- Recursive folder scanning

### 📊 Progress Tracking
- Real-time progress bar
- File count and completion status
- Detailed status messages

### 🛡️ Error Handling
- Clear error messages for failed conversions
- Continues processing even if some files fail
- Comprehensive error reporting

## 🎯 Supported File Types

### Input
- **Adobe Photoshop PSD files** (.psd)
- Standard RGB color mode
- Basic layer compositions
- Common layer effects

### Output
- **PNG files** (.png)
- High quality preservation
- Transparency support
- Lossless compression

## 🔧 Technical Details

### Performance
- Multi-threaded processing for faster conversion
- Memory-efficient handling of large files
- Optimized for batch operations

### Compatibility
- Works with most standard PSD files
- Handles various color modes
- Supports transparency

### Limitations
- Complex layer effects may not convert perfectly
- Some advanced PSD features are not supported
- Very large files may take longer to process

## 🐛 Troubleshooting

### Common Issues

**"No PSD files found"**
- Ensure your input folder contains `.psd` files
- Check that files are valid Adobe Photoshop files
- Verify folder permissions

**Conversion fails for some files**
- Some PSD files may have unsupported features
- Try opening in Photoshop and saving as PNG first
- Check file size and complexity

**Permission errors**
- Make sure you have write access to the output folder
- Run as administrator if necessary
- Check disk space availability

### Error Messages

**"Failed to parse PSD file"**
- The PSD file may be corrupted or unsupported
- Try opening in Photoshop first
- Consider saving as PNG from Photoshop

**"Failed to save PNG file"**
- Check disk space
- Verify write permissions
- Ensure output folder exists

## 💡 Tips for Best Results

1. **File Preparation**:
   - Flatten complex layers before conversion
   - Use standard RGB color mode
   - Remove unnecessary layers

2. **Batch Organization**:
   - Organize PSD files in folders
   - Use consistent naming conventions
   - Keep similar files together

3. **Performance Optimization**:
   - Process files in smaller batches for large collections
   - Close other applications to free up memory
   - Use SSD storage for faster I/O

## 📁 Example Workflow

```
1. Create folders:
   ├── my-psd-files/          # Input folder
   │   ├── design1.psd
   │   ├── design2.psd
   │   └── subfolder/
   │       └── design3.psd
   └── converted-pngs/        # Output folder

2. Run the application
3. Drag "my-psd-files" onto the app
4. Select "converted-pngs" as output
5. Click "Start Conversion"

Result:
   └── converted-pngs/
       ├── design1.png
       ├── design2.png
       └── subfolder/
           └── design3.png
```

## 🎉 Success!

After conversion completes, you'll see:
- ✅ "Successfully converted X files!" message
- All PNG files in your output folder
- Preserved folder structure
- High-quality converted images

---

**Enjoy converting your PSD files to PNG format! 🚀**
