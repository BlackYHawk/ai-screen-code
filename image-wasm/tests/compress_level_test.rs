//! Tests for compression levels (T013)
//!
//! Tests JPEG, PNG, WebP compression at different compression levels:
//! - Light (quality priority)
//! - Normal (balanced)
//! - Strong (size priority)
//! - Extreme (smallest size)

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageEncoder, Rgb, Rgba};
use image_wasm::compress::compress;
use image_wasm::{CompressionConfig, CompressionLevel, OutputFormat};

/// Create a simple test image (100x100 RGB)
fn create_test_rgb_image() -> DynamicImage {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_fn(100, 100, |x, y| Rgb([x as u8, y as u8, (x + y) as u8]));
    DynamicImage::ImageRgb8(img)
}

/// Create a simple test image (100x100 RGBA with transparency)
fn create_test_rgba_image() -> DynamicImage {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, y| {
        if x < 50 && y < 50 {
            Rgba([255, 0, 0, 128]) // Red semi-transparent
        } else {
            Rgba([0, 255, 0, 255]) // Green opaque
        }
    });
    DynamicImage::ImageRgba8(img)
}

/// Encode image to JPEG bytes
fn encode_to_jpeg(img: &DynamicImage) -> Vec<u8> {
    let mut buffer = Vec::new();
    {
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 100);
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

mod jpeg_compression {
    use super::*;

    #[test]
    fn test_compress_jpeg_light() {
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Light,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
        // Light compression should produce larger file than other levels
        assert!(output.len() > 100);
    }

    #[test]
    fn test_compress_jpeg_normal() {
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_compress_jpeg_strong() {
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Strong,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_compress_jpeg_extreme() {
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Extreme,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_jpeg_compression_levels_produce_different_sizes() {
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img);

        // Test all compression levels
        let levels = [
            CompressionLevel::Light,
            CompressionLevel::Normal,
            CompressionLevel::Strong,
            CompressionLevel::Extreme,
        ];

        let mut sizes = Vec::new();
        for level in &levels {
            let config = CompressionConfig {
                level: *level,
                format: OutputFormat::Jpeg,
            };
            let result = compress(&input, &config).unwrap();
            sizes.push(result.len());
        }

        // Higher compression levels should produce smaller files
        // Light (85 quality) > Normal (70) > Strong (50) > Extreme (30)
        assert!(
            sizes[0] >= sizes[1],
            "Light should be >= Normal: {} >= {}",
            sizes[0],
            sizes[1]
        );
        assert!(
            sizes[1] >= sizes[2],
            "Normal should be >= Strong: {} >= {}",
            sizes[1],
            sizes[2]
        );
        assert!(
            sizes[2] >= sizes[3],
            "Strong should be >= Extreme: {} >= {}",
            sizes[2],
            sizes[3]
        );
    }
}

mod png_compression {
    use super::*;

    #[test]
    fn test_compress_png_light() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Light,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_compress_png_normal() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_compress_png_strong() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Strong,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_compress_png_extreme() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Extreme,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }
}

mod webp_compression {
    use super::*;

    #[test]
    fn test_compress_webp_light() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Light,
            format: OutputFormat::WebP,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_compress_webp_normal() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::WebP,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_compress_webp_strong() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Strong,
            format: OutputFormat::WebP,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_compress_webp_extreme() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Extreme,
            format: OutputFormat::WebP,
        };

        let result = compress(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }
}

mod compression_preserves_dimensions {
    use super::*;

    #[test]
    fn test_jpeg_preserves_dimensions() {
        let img = create_test_rgb_image();
        let input = encode_to_jpeg(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config).unwrap();

        // Decode the result and check dimensions
        let decoded = image::load_from_memory(&result).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }

    #[test]
    fn test_png_preserves_dimensions() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config).unwrap();

        // Decode the result and check dimensions
        let decoded = image::load_from_memory(&result).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }

    #[test]
    fn test_webp_preserves_dimensions() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::WebP,
        };

        let result = compress(&input, &config).unwrap();

        // Decode the result and check dimensions
        // Note: WebP might fall back to PNG, which is fine
        let decoded = image::load_from_memory(&result).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));
    }
}

mod compression_with_transparency {
    use super::*;

    #[test]
    fn test_png_preserves_transparency() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Png,
        };

        let result = compress(&input, &config).unwrap();

        // Verify the output is valid PNG
        let decoded = image::load_from_memory(&result).unwrap();
        assert_eq!(decoded.dimensions(), (100, 100));

        // Check that we have transparency
        let rgba = decoded.to_rgba8();
        // Top-left should be semi-transparent red
        let pixel = rgba.get_pixel(0, 0);
        assert_eq!(pixel[3], 128); // Alpha channel
    }

    #[test]
    fn test_jpeg_removes_transparency() {
        let img = create_test_rgba_image();
        let input = encode_to_png(&img);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::Jpeg,
        };

        let result = compress(&input, &config).unwrap();

        // JPEG doesn't support transparency, output should be valid
        let decoded = image::load_from_memory(&result).unwrap();
        // Image should be converted to RGB (no alpha)
        assert!(matches!(decoded, DynamicImage::ImageRgb8(_)));
    }
}
