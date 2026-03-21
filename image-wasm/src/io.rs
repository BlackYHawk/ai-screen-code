use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader};
use std::io::Cursor;

use crate::error::ImageError;
use crate::types::OutputFormat;

/// Detect image format from magic bytes
pub fn detect_format(data: &[u8]) -> Result<ImageFormat, ImageError> {
    if data.len() < 4 {
        return Err(ImageError::InvalidFormat("Data too short".to_string()));
    }

    // PNG
    if data.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        return Ok(ImageFormat::Png);
    }

    // JPEG
    if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Ok(ImageFormat::Jpeg);
    }

    // WebP
    if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" {
        return Ok(ImageFormat::WebP);
    }

    // GIF
    if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
        return Ok(ImageFormat::Gif);
    }

    // BMP
    if data.starts_with(b"BM") {
        return Ok(ImageFormat::Bmp);
    }

    Err(ImageError::UnsupportedFormat(
        "Unknown image format".to_string(),
    ))
}

/// Load image from bytes
pub fn load_image(data: &[u8]) -> Result<DynamicImage, ImageError> {
    let format = detect_format(data)?;

    ImageReader::with_format(Cursor::new(data), format)
        .decode()
        .map_err(|e| ImageError::ProcessingError(format!("Failed to decode image: {}", e)))
}

/// Get image dimensions
pub fn get_dimensions(img: &DynamicImage) -> (u32, u32) {
    img.dimensions()
}

/// Save image to bytes with specified format
pub fn save_image(img: &DynamicImage, format: OutputFormat) -> Result<Vec<u8>, ImageError> {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    let image_format = match format {
        OutputFormat::Jpeg => ImageFormat::Jpeg,
        OutputFormat::Png => ImageFormat::Png,
        OutputFormat::WebP => ImageFormat::WebP,
        OutputFormat::Bmp => ImageFormat::Bmp,
        OutputFormat::Gif => ImageFormat::Gif,
        OutputFormat::Tiff => ImageFormat::Tiff,
    };

    img.write_to(&mut cursor, image_format)
        .map_err(|e| ImageError::ProcessingError(format!("Failed to encode image: {}", e)))?;

    Ok(buffer)
}
