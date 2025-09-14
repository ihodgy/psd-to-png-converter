use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("No PSD files found in the selected folder")]
    NoPsdFilesFound,
    
    #[error("Invalid file format: {0}")]
    InvalidFileFormat(String),
    
    #[error("File system error: {0}")]
    FileSystemError(String),
    
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    
    #[error("Generic error: {0}")]
    Generic(#[from] anyhow::Error),
}
