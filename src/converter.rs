use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use image::{ImageFormat, DynamicImage};
use anyhow::{Result, Context};
use tokio::task;
use crate::error::AppError;

pub struct PsdConverter {
    progress_callback: Option<Box<dyn Fn(f32, String) + Send + Sync>>,
}

impl PsdConverter {
    pub fn new() -> Self {
        Self {
            progress_callback: None,
        }
    }

    pub fn set_progress_callback<F>(&mut self, callback: F)
    where
        F: Fn(f32, String) + Send + Sync + 'static,
    {
        self.progress_callback = Some(Box::new(callback));
    }

    pub async fn convert_folder(&mut self, input_path: PathBuf, output_path: PathBuf) -> Result<()> {
        self.notify_progress(0.0, "Scanning for PSD files...".to_string());

        // Ensure output directory exists
        fs::create_dir_all(&output_path)
            .context("Failed to create output directory")?;

        // Find all PSD files
        let psd_files = self.find_psd_files(&input_path)?;
        
        if psd_files.is_empty() {
            return Err(AppError::NoPsdFilesFound.into());
        }

        let total_files = psd_files.len();
        let mut converted_count = 0;
        let mut errors = Vec::new();

        self.notify_progress(0.0, format!("Found {} PSD files to convert", total_files));

        for (index, psd_file) in psd_files.iter().enumerate() {
            let relative_path = psd_file.strip_prefix(&input_path)
                .context("Failed to get relative path")?;
            
            let output_file = output_path.join(relative_path)
                .with_extension("png");

            // Create output subdirectories if needed
            if let Some(parent) = output_file.parent() {
                fs::create_dir_all(parent)
                    .context("Failed to create output subdirectory")?;
            }

            match self.convert_single_file(psd_file, &output_file).await {
                Ok(_) => {
                    converted_count += 1;
                    let progress = (index + 1) as f32 / total_files as f32;
                    let message = format!("Converted {}/{} files", converted_count, total_files);
                    self.notify_progress(progress, message);
                }
                Err(e) => {
                    errors.push((psd_file.clone(), e.to_string()));
                    let message = format!("Error converting {}: {}", 
                        psd_file.file_name().unwrap().to_string_lossy(), 
                        e);
                    self.notify_progress((index + 1) as f32 / total_files as f32, message);
                }
            }
        }

        if errors.is_empty() {
            self.notify_progress(1.0, format!("Successfully converted {} files!", converted_count));
        } else {
            let message = format!("Converted {} files, {} errors occurred", converted_count, errors.len());
            self.notify_progress(1.0, message);
            
            // Log errors
            for (file, error) in errors {
                eprintln!("Failed to convert {}: {}", file.display(), error);
            }
        }

        Ok(())
    }

    async fn convert_single_file(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        let input_path = input_path.to_path_buf();
        let output_path = output_path.to_path_buf();

        // Run the conversion in a blocking task
        task::spawn_blocking(move || {
            Self::convert_psd_to_png_sync(&input_path, &output_path)
        }).await
        .context("Conversion task failed")?
        .context("Failed to convert PSD to PNG")?;

        Ok(())
    }

    pub fn convert_psd_to_png_sync(input_path: &Path, output_path: &Path) -> Result<()> {
        // Try to parse as PSD using psd crate
        let psd_data = fs::read(input_path)
            .context("Failed to read PSD file")?;
            
        let psd = match psd::Psd::from_bytes(&psd_data) {
            Ok(psd) => psd,
            Err(_) => {
                // If PSD parsing fails, try to read as regular image
                return Self::convert_as_image_file(input_path, output_path);
            }
        };

        // Get the flattened image from PSD
        let width = psd.width();
        let height = psd.height();
        let rgba_data = psd.rgba();
        
        // Convert to DynamicImage
        let img = DynamicImage::ImageRgba8(
            image::ImageBuffer::from_raw(
                width,
                height,
                rgba_data,
            ).context("Failed to create image buffer")?
        );

        // Save as PNG
        img.save_with_format(output_path, ImageFormat::Png)
            .context("Failed to save PNG file")?;

        Ok(())
    }

    fn convert_as_image_file(input_path: &Path, output_path: &Path) -> Result<()> {
        // Try to read as image (this will work for some PSD files that are compatible)
        let img = image::open(input_path)
            .context("Failed to parse PSD file. Make sure the file is a valid PSD or try saving as PNG first.")?;

        // Save as PNG
        img.save_with_format(output_path, ImageFormat::Png)
            .context("Failed to save PNG file")?;

        Ok(())
    }

    fn find_psd_files(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut psd_files = Vec::new();

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(extension) = entry.path().extension() {
                if extension.to_string_lossy().to_lowercase() == "psd" {
                    psd_files.push(entry.path().to_path_buf());
                }
            }
        }

        Ok(psd_files)
    }

    fn notify_progress(&self, progress: f32, message: String) {
        if let Some(ref callback) = self.progress_callback {
            callback(progress, message);
        }
    }
}

impl Default for PsdConverter {
    fn default() -> Self {
        Self::new()
    }
}
