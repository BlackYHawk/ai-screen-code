use std::fmt;

#[derive(Debug)]
pub enum ImageError {
    InvalidFormat(String),
    UnsupportedFormat(String),
    IoError(String),
    ProcessingError(String),
    InvalidParameter(String),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ImageError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
            ImageError::IoError(msg) => write!(f, "IO error: {}", msg),
            ImageError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            ImageError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
        }
    }
}

impl std::error::Error for ImageError {}
