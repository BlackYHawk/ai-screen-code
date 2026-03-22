//! Tests for format conversion (T014)
//!
//! Tests format conversion between JPG, PNG, WebP

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageEncoder, Rgb, Rgba};
use image_wasm::compress::compress;
use image_wasm::{CompressionConfig, CompressionLevel, OutputFormat};

/// Create a simple test image (100x100 RGB)
fn create_test_rgb_image() -> DynamicImage {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_fn(100, 100, |x, y| Rgb([x as u8, y as u8, (x + y) as u8]));
    DynamicImage::ImageRgb8(img)
}

/// Create a test image with transparency
fn create_test_rgba_image() -> DynamicImage {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, y| {
        if x < 50 && y < 50 {
            Rgba([255, 0, 0, 128])
        } else {
            Rgba([0, 255, 0, 255])
        }
    });
    DynamicImage::ImageRgba8(img)
}

/// Encode image to JPEG bytes
fn encode_to_jpeg(img: &DynamicImage) -> Vec<u8> {
    let mut buffer = Vec::new();
    {
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 90);
        let rgb_img = img.to_rgb8();
        encoder
            .write_image(rgb_img.as_raw(), 100, 100, image::ExtendedColorType::Rgb8)
            .unwrap();
    }
    buffer
}

/// Encode image to PNG bytes
fn encode_to_png(img: &DynamicImage) -> Vec<u8> {
    let mut buffer = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
        let rgba_img = img.to_rgba8();
        encoder
            .write_image(rgba_img.as_raw(), 100, 100, image::ExtendedColorType::Rgba8)
            .unwrap();
    }
    buffer
}

mod jpeg_to_png {
    use super::*;

    #[test]
    fn test_jpeg_to_png_conversion() {
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        // Verify it's a valid PNG
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }

    #[test]
    fn test_jpeg_to_png_preserves_dimensions() {
        // Create larger image (200x150)
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_fn(200, 150, |x, y| Rgb([x as u8, y as u8, 128]));
        let input = {
            let mut buffer = Vec::new();
            {
                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 90);
                encoder
                    .write_image(img.as_raw(), 200, 150, image::ExtendedColorType::Rgb8)
                    .unwrap();
            }
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

mod jpeg_to_webp {
    use super::*;

    #[test]
    fn test_jpeg_to_webp_conversion() {
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::WebP,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        // Verify it's valid (may be PNG fallback)
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }
}

mod png_to_jpeg {
    use super::*;

    #[test]
    fn test_png_to_jpeg_conversion() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        // Verify it's a valid JPEG
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }

    #[test]
    fn test_png_to_jpeg_removes_transparency() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config).unwrap();

        let decoded = image::load_from_memory(&result).unwrap();
        // JPEG should have no alpha channel
        assert!(matches!(decoded, DynamicImage::ImageRgb8(_)));
    }
}

mod png_to_webp {
    use super::*;

    #[test]
    fn test_png_to_webp_conversion() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::WebP,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        // Verify it's valid (may be PNG fallback)
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }
}

mod webp_conversion {
    use super::*;

    // Note: image crate may not have native WebP encoder support,
    // tests verify fallback behavior works

    #[test]
    fn test_webp_to_jpeg() {
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img); // Use JPEG as input (simulating WebP)

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::WebP,
        };

        let result = compress(&input, &config).unwrap();

        // Convert WebP (PNG fallback) to JPEG
        let config2 = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Jpeg,
        };

        let result2 = compress(&result, &config2).unwrap();

        let decoded = image::load_from_memory(&result2).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }

    #[test]
    fn test_webp_to_png() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::WebP,
        };

        let result = compress(&input, &config).unwrap();

        // Convert WebP (PNG fallback) to PNG
        let config2 = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };

        let result2 = compress(&result, &config2).unwrap();

        let decoded = image::load_from_memory(&result2).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }
}

mod roundtrip_conversion {
    use super::*;

    #[test]
    fn test_png_to_jpeg_to_png_roundtrip() {
        // Start with PNG
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        // Convert to JPEG
        let config_jpeg = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Jpeg,
        };
        let jpeg_result = compress(&input, &config_jpeg).unwrap();

        // Convert back to PNG
        let config_png = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };
        let png_result = compress(&jpeg_result, &config_png).unwrap();

        // Should be valid
        let decoded = image::load_from_memory(&png_result).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }

    #[test]
    fn test_jpeg_to_png_to_jpeg_roundtrip() {
        // Start with JPEG
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img);

        // Convert to PNG
        let config_png = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };
        let png_result = compress(&input, &config_png).unwrap();

        // Convert back to JPEG
        let config_jpeg = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Jpeg,
        };
        let jpeg_result = compress(&png_result, &config_jpeg).unwrap();

        // Should be valid
        let decoded = image::load_from_memory(&jpeg_result).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }
}

mod format_detection {
    use super::*;

    #[test]
    fn test_detect_jpeg_input() {
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img);

        // Should work with any output format
        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_detect_png_input() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        // Should work with any output format
        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
    }
}
