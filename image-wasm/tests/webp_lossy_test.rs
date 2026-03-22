//! Unit tests for lossy WebP compression

use image::ImageEncoder;
use image_wasm::compress::compress;
use image_wasm::{CompressionConfig, CompressionLevel, OutputFormat};

/// Test lossy WebP compression at different quality levels
mod webp_lossy_compression {
    use super::*;

    /// Test light quality WebP compression
    #[test]
    fn test_webp_lossy_light_quality() {
        // Create a simple test image (10x10 red pixels)
        let img_data = create_test_image(10, 10, [255, 0, 0, 255]);

        let config = CompressionConfig {
            level: CompressionLevel::Light,
            format: OutputFormat::WebP,
        };

        let result = compress(&img_data, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        // Light quality should produce reasonable size
        assert!(!output.is_empty());
        // WebP header is at least 20 bytes
        assert!(output.len() > 20);
    }

    /// Test normal quality WebP compression
    #[test]
    fn test_webp_lossy_normal_quality() {
        let img_data = create_test_image(10, 10, [0, 255, 0, 255]);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::WebP,
        };

        let result = compress(&img_data, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    /// Test strong quality WebP compression
    #[test]
    fn test_webp_lossy_strong_quality() {
        let img_data = create_test_image(10, 10, [0, 0, 255, 255]);

        let config = CompressionConfig {
            level: CompressionLevel::Strong,
            format: OutputFormat::WebP,
        };

        let result = compress(&img_data, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    /// Test extreme quality WebP compression (smallest size)
    #[test]
    fn test_webp_lossy_extreme_quality() {
        let img_data = create_test_image(10, 10, [255, 255, 0, 255]);

        let config = CompressionConfig {
            level: CompressionLevel::Extreme,
            format: OutputFormat::WebP,
        };

        let result = compress(&img_data, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    /// Test that different quality levels produce different file sizes
    #[test]
    fn test_webp_quality_levels_produce_different_sizes() {
        let img_data = create_test_image(50, 50, [128, 128, 128, 255]);

        // Light quality (high quality, larger file)
        let light_config = CompressionConfig {
            level: CompressionLevel::Light,
            format: OutputFormat::WebP,
        };
        let light_result = compress(&img_data, &light_config).unwrap();

        // Extreme quality (low quality, smaller file)
        let extreme_config = CompressionConfig {
            level: CompressionLevel::Extreme,
            format: OutputFormat::WebP,
        };
        let extreme_result = compress(&img_data, &extreme_config).unwrap();

        // Light should produce larger or equal file than extreme
        assert!(
            light_result.len() >= extreme_result.len(),
            "Light quality ({}) should be >= extreme quality ({})",
            light_result.len(),
            extreme_result.len()
        );
    }

    /// Test WebP compression preserves dimensions
    #[test]
    fn test_webp_preserves_dimensions() {
        let img_data = create_test_image(100, 50, [255, 0, 255, 255]);

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::WebP,
        };

        let result = compress(&img_data, &config);
        assert!(result.is_ok());

        // Decode and verify dimensions using image crate
        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.width(), 100);
        assert_eq!(decoded.height(), 50);
    }

    /// Test WebP compression with transparency
    #[test]
    fn test_webp_with_transparency() {
        // Create image with transparency
        let mut img = image::RgbaImage::new(20, 20);
        for y in 0..20 {
            for x in 0..20 {
                let pixel = if x < 10 {
                    image::Rgba([255, 0, 0, 255]) // Red, opaque
                } else {
                    image::Rgba([0, 0, 0, 0]) // Transparent
                };
                img.put_pixel(x, y, pixel);
            }
        }

        let mut buf = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut buf);
        encoder
            .write_image(img.as_raw(), 20, 20, image::ExtendedColorType::Rgba8)
            .unwrap();

        let config = CompressionConfig {
            level: CompressionLevel::Normal,
            format: OutputFormat::WebP,
        };

        let result = compress(&buf, &config);
        assert!(result.is_ok());
    }
}

/// Helper function to create a test image with specified color
fn create_test_image(width: u32, height: u32, color: [u8; 4]) -> Vec<u8> {
    let mut img = image::RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            img.put_pixel(x, y, image::Rgba(color));
        }
    }

    let mut buf = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    encoder
        .write_image(img.as_raw(), width, height, image::ExtendedColorType::Rgba8)
        .unwrap();

    buf
}
