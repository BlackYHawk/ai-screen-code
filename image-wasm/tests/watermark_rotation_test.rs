//! Tests for watermark rotation transformation (T026)
//!
//! Tests rotation angles 0-360 degrees
//!
//! Note: Current implementation uses placeholder rendering.
//! Tests verify rotation parameter is accepted and processed.

use image::{DynamicImage, ImageBuffer, Rgb, ImageEncoder};
use image_wasm::watermark;
use image_wasm::types::{WatermarkConfig, WatermarkPosition, FontWeight};

/// Create a simple test image (200x200 RGB)
fn create_test_image() -> DynamicImage {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_fn(200, 200, |x, y| Rgb([x as u8, y as u8, 128]));
    DynamicImage::ImageRgb8(img)
}

/// Encode image to PNG bytes
fn encode_to_png(img: &DynamicImage) -> Vec<u8> {
    let mut buffer = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
        let rgba_img = img.to_rgba8();
        encoder
            .write_image(rgba_img.as_raw(), 200, 200, image::ExtendedColorType::Rgba8)
            .unwrap();
    }
    buffer
}

mod rotation_tests {
    use super::*;
    use image::GenericImageView;

    /// Test rotation 0 degrees (no rotation)
    #[test]
    fn test_rotation_0() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::Center,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    /// Test rotation 45 degrees
    #[test]
    fn test_rotation_45() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::Center,
            rotation: 45.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    /// Test rotation 90 degrees
    #[test]
    fn test_rotation_90() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::Center,
            rotation: 90.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    /// Test rotation 180 degrees
    #[test]
    fn test_rotation_180() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::Center,
            rotation: 180.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    /// Test rotation 270 degrees
    #[test]
    fn test_rotation_270() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::Center,
            rotation: 270.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    /// Test rotation 360 degrees (full circle, same as 0)
    #[test]
    fn test_rotation_360() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::Center,
            rotation: 360.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    /// Test rotation with different positions
    #[test]
    fn test_rotation_with_different_positions() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let positions = [
            WatermarkPosition::TopLeft,
            WatermarkPosition::Center,
            WatermarkPosition::BottomRight,
        ];

        for position in &positions {
            let config = WatermarkConfig {
                text: "WM".to_string(),
                font_size: 24,
                color: "#FF0000".to_string(),
                position: *position,
                rotation: 45.0,
                font_weight: FontWeight::Normal,
            };

            let result = watermark::add_watermark(&input, &config);
            assert!(result.is_ok());
        }
    }

    /// Test rotation at corner positions
    #[test]
    fn test_rotation_at_corners() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let positions = [
            WatermarkPosition::TopLeft,
            WatermarkPosition::TopRight,
            WatermarkPosition::BottomLeft,
            WatermarkPosition::BottomRight,
        ];

        for position in &positions {
            let config = WatermarkConfig {
                text: "WM".to_string(),
                font_size: 24,
                color: "#FF0000".to_string(),
                position: *position,
                rotation: 30.0,
                font_weight: FontWeight::Normal,
            };

            let result = watermark::add_watermark(&input, &config);
            assert!(result.is_ok());
        }
    }

    /// Test rotation 0 and 360 produce same dimensions
    #[test]
    fn test_rotation_0_and_360_same_dimensions() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config_0 = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::Center,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let config_360 = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::Center,
            rotation: 360.0,
            font_weight: FontWeight::Normal,
        };

        let result_0 = watermark::add_watermark(&input, &config_0).unwrap();
        let result_360 = watermark::add_watermark(&input, &config_360).unwrap();

        // Both should produce valid PNG
        let decoded_0 = image::load_from_memory(&result_0).unwrap();
        let decoded_360 = image::load_from_memory(&result_360).unwrap();

        // Dimensions should match
        assert_eq!(decoded_0.dimensions(), decoded_360.dimensions());
    }

    /// Test rotation intermediate angles
    #[test]
    fn test_rotation_intermediate_angles() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let angles = [15.0, 30.0, 60.0, 120.0, 135.0, 225.0, 315.0];

        for angle in &angles {
            let config = WatermarkConfig {
                text: "WM".to_string(),
                font_size: 24,
                color: "#FF0000".to_string(),
                position: WatermarkPosition::Center,
                rotation: *angle,
                font_weight: FontWeight::Normal,
            };

            let result = watermark::add_watermark(&input, &config);
            assert!(result.is_ok(), "Rotation {} failed", angle);
        }
    }

    /// Test rotation with different font sizes
    #[test]
    fn test_rotation_with_different_font_sizes() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let font_sizes = [12, 16, 24, 32, 48];

        for font_size in &font_sizes {
            let config = WatermarkConfig {
                text: "WM".to_string(),
                font_size: *font_size,
                color: "#FF0000".to_string(),
                position: WatermarkPosition::Center,
                rotation: 45.0,
                font_weight: FontWeight::Normal,
            };

            let result = watermark::add_watermark(&input, &config);
            assert!(result.is_ok());
        }
    }
}
