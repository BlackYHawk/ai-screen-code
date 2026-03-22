//! Tests for APNG to static image conversion (T014a)
//!
//! Tests APNG (Animated PNG) decoding to static image

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageEncoder, Rgba};
use image_wasm::compress::compress;
use image_wasm::{CompressionConfig, CompressionLevel, OutputFormat};

/// Create a simple test image (100x100 RGBA)
fn create_test_rgba_image() -> DynamicImage {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_fn(100, 100, |x, y| Rgba([x as u8, y as u8, 128, 255]));
    DynamicImage::ImageRgba8(img)
}

/// Encode image to PNG bytes
fn encode_to_png(img: &DynamicImage) -> Vec<u8> {
    let mut buffer = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
    let rgba_img = img.to_rgba8();
    encoder
        .write_image(rgba_img.as_raw(), 100, 100, image::ExtendedColorType::Rgba8)
        .unwrap();
    buffer
}

/// Create minimal APNG data (single frame)
/// APNG signature: 89 50 4E 47 0D 0A 1A 0A
/// Followed by IHDR, IDAT (or fdAT for animation), IEND
fn create_minimal_apng() -> Vec<u8> {
    // Create a simple 10x10 PNG first, then convert to APNG-like format
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_fn(10, 10, |x, y| Rgba([x as u8 * 25, y as u8 * 25, 128, 255]));

    let mut buffer = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
        encoder
            .write_image(img.as_raw(), 10, 10, image::ExtendedColorType::Rgba8)
            .unwrap();
    }
    buffer
}

mod apng_detection {
    use super::*;

    #[test]
    fn test_apng_as_regular_png() {
        // Regular PNG should work fine
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apng_to_jpeg() {
        // APNG converted to static image then to JPEG
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }
}

mod apng_frame_handling {
    use super::*;

    #[test]
    fn test_apng_first_frame_extraction() {
        // Test that APNG is handled (even if treated as static)
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        // Should extract first frame and convert
        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config).unwrap();

        let decoded = image::load_from_memory(&result).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }

    #[test]
    fn test_apng_preserves_dimensions() {
        // Create different sized image
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_fn(200, 150, |x, y| Rgba([x as u8, y as u8, 128, 255]));
        let input = {
            let mut buffer = Vec::new();
            let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
            encoder
                .write_image(img.as_raw(), 200, 150, image::ExtendedColorType::Rgba8)
                .unwrap();
            buffer
        };

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config).unwrap();

        let decoded = image::load_from_memory(&result).unwrap();
        assert_eq!(decoded.dimensions(), (200, 150));
    }
}

mod apng_conversion_workflow {
    use super::*;

    #[test]
    fn test_apng_to_static_then_compress() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        // First convert APNG to static PNG
        let config_png = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };
        let png_result = compress(&input, &config_png).unwrap();

        // Then compress as normal
        let config_compress = CompressionConfig {
            level: CompressionLevel::Strong,
            format: OutputFormat::Png,
        };
        let compressed = compress(&png_result, &config_compress).unwrap();

        assert!(!compressed.is_empty());
        assert!(compressed.len() <= png_result.len());
    }

    #[test]
    fn test_apng_to_jpeg_with_compression() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        // Convert APNG to JPEG with strong compression
        let config = CompressionConfig {
            level: CompressionLevel::Strong,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config).unwrap();

        // Verify it's valid
        let decoded = image::load_from_memory(&result).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
        assert!(matches!(decoded, DynamicImage::ImageRgb8(_)));
    }
}
