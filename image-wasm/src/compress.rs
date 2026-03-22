use crate::error::ImageError;
use crate::io;
use crate::types::{CompressionConfig, CompressionLevel, OutputFormat};
use image::{DynamicImage, ImageEncoder};
use std::io::Write;

/// Get JPEG quality setting from compression level
fn jpeg_quality(level: &CompressionLevel) -> u8 {
    match level {
        CompressionLevel::Light => 80,
        CompressionLevel::Normal => 60,
        CompressionLevel::Strong => 40,
        CompressionLevel::Extreme => 20,
    }
}

/// Get PNG compression level from zlib compression (0-9)
fn png_zlib_level(level: &CompressionLevel) -> u32 {
    match level {
        CompressionLevel::Light => 3,   // Fast, less compression
        CompressionLevel::Normal => 6,  // Default balanced
        CompressionLevel::Strong => 8,  // Higher compression
        CompressionLevel::Extreme => 9,  // Maximum compression
    }
}

/// Compress image with specified configuration
pub fn compress(data: &[u8], config: &CompressionConfig) -> Result<Vec<u8>, ImageError> {
    let img = io::load_image(data)?;

    let output_data = match config.format {
        OutputFormat::Jpeg => compress_to_jpeg(&img, &config.level),
        OutputFormat::Png => compress_to_png(&img, &config.level),
        OutputFormat::WebP => io::save_image(&img, OutputFormat::WebP),

        OutputFormat::Bmp => io::save_image(&img, OutputFormat::Bmp),
        OutputFormat::Gif => io::save_image(&img, OutputFormat::Gif),
        OutputFormat::Tiff => io::save_image(&img, OutputFormat::Tiff),
    }?;

    Ok(output_data)
}

fn compress_to_jpeg(img: &DynamicImage, level: &CompressionLevel) -> Result<Vec<u8>, ImageError> {
    let quality = jpeg_quality(level);

    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    let mut buffer = Vec::new();
    {
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, quality);
        encoder
            .write_image(
                rgb_img.as_raw(),
                width,
                height,
                image::ExtendedColorType::Rgb8,
            )
            .map_err(|e| ImageError::ProcessingError(format!("JPEG encoding failed: {}", e)))?;
    }

    Ok(buffer)
}

fn compress_to_png(img: &DynamicImage, level: &CompressionLevel) -> Result<Vec<u8>, ImageError> {
    let zlib_level = png_zlib_level(level);
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    // Use png crate directly for proper compression control
    let mut buffer = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut buffer, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Best);
        encoder.set_filter(png::FilterType::Sub);

        let mut writer = encoder
            .write_header()
            .map_err(|e| ImageError::ProcessingError(format!("PNG header failed: {}", e)))?;

        // Convert to raw bytes with filter bytes
        let raw_data = rgba_img.as_raw();
        let mut filtered_data = Vec::with_capacity((width * 4 + 1) as usize * height as usize);

        for row in raw_data.chunks_exact((width * 4) as usize) {
            filtered_data.push(0); // No filter
            filtered_data.extend_from_slice(row);
        }

        // Compress with specified zlib level
        let compressed = compress_zlib(&filtered_data, zlib_level)?;
        writer
            .write_image_data(&compressed)
            .map_err(|e| ImageError::ProcessingError(format!("PNG write failed: {}", e)))?;
    }

    Ok(buffer)
}

/// Compress data with zlib at specified level
fn compress_zlib(data: &[u8], level: u32) -> Result<Vec<u8>, ImageError> {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;

    let compression = match level {
        0..=3 => Compression::new(1),
        4..=6 => Compression::new(4),
        7..=8 => Compression::new(7),
        _ => Compression::new(9),
    };

    let mut encoder = ZlibEncoder::new(Vec::new(), compression);
    encoder
        .write_all(data)
        .map_err(|e| ImageError::ProcessingError(format!("Zlib compression failed: {}", e)))?;

    encoder
        .finish()
        .map_err(|e| ImageError::ProcessingError(format!("Zlib finish failed: {}", e)))
}
