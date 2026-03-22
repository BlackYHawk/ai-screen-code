//! Tests for watermark position calculation (T025)
//!
//! Tests 9 position placements: corners, edges, and center
//!
//! Position layout:
//! +----------+----------+----------+
//! | TopLeft  | TopCenter| TopRight |
//! +----------+----------+----------+
//! |MiddleLeft|  Center  |MiddleRight|
//! +----------+----------+----------+
//! |BottomLeft|BottomCen |BottomRight|
//! +----------+----------+----------+

use image::{DynamicImage, ImageBuffer, Rgb, ImageEncoder, GenericImageView};
use image_wasm::watermark;
use image_wasm::types::{WatermarkConfig, WatermarkPosition, FontWeight};

/// Create a simple test image (200x200 RGB)
fn create_test_image() -> DynamicImage {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_fn(200, 200, |x, y| Rgb([x as u8, y as u8, 128]));
    DynamicImage::ImageRgb8(img)
}

/// Create a specific size test image
fn create_test_image_with_size(width: u32, height: u32) -> DynamicImage {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_fn(width, height, |x, y| Rgb([x as u8, y as u8, 128]));
    DynamicImage::ImageRgb8(img)
}

/// Encode image to PNG bytes
fn encode_to_png(img: &DynamicImage) -> Vec<u8> {
    let (width, height) = img.dimensions();
    let mut buffer = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
        let rgba_img = img.to_rgba8();
        encoder
            .write_image(rgba_img.as_raw(), width, height, image::ExtendedColorType::Rgba8)
            .unwrap();
    }
    buffer
}

mod position_tests {
    use super::*;

    /// Helper to check if a pixel has been modified (not original gradient color)
    fn is_pixel_modified(pixel: &image::Rgba<u8>, expected_r: u8, expected_g: u8) -> bool {
        // Original was Rgb([x, y, 128]) -> Rgba([x, y, 128, 255])
        // Watermark adds white background with color border, so it should be different
        pixel.0[0] != expected_r || pixel.0[1] != expected_g || pixel.0[2] != 128
    }

    /// Test TopLeft position - watermark should be in top-left corner
    #[test]
    fn test_position_top_left() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::TopLeft,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();

        // Verify dimensions preserved
        assert_eq!(decoded.dimensions(), (200, 200));

        let rgba = decoded.to_rgba8();

        // Check watermark area in top-left (around position 20-50)
        let modified = is_pixel_modified(rgba.get_pixel(30, 30), 30, 30);
        assert!(modified, "Top-left area should be modified by watermark");
    }

    /// Test TopCenter position - watermark should be at top edge center
    #[test]
    fn test_position_top_center() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#00FF00".to_string(),
            position: WatermarkPosition::TopCenter,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (200, 200));

        let rgba = decoded.to_rgba8();

        // Check watermark area at top-center (around x=100)
        let modified = is_pixel_modified(rgba.get_pixel(100, 30), 100, 30);
        assert!(modified, "Top-center area should be modified by watermark");
    }

    /// Test TopRight position - watermark should be in top-right corner
    #[test]
    fn test_position_top_right() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#0000FF".to_string(),
            position: WatermarkPosition::TopRight,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (200, 200));

        let rgba = decoded.to_rgba8();

        // Check watermark area at top-right (around x=170)
        let modified = is_pixel_modified(rgba.get_pixel(170, 30), 170, 30);
        assert!(modified, "Top-right area should be modified by watermark");
    }

    /// Test MiddleLeft position - watermark should be at left edge center
    #[test]
    fn test_position_middle_left() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FFFF00".to_string(),
            position: WatermarkPosition::MiddleLeft,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (200, 200));

        let rgba = decoded.to_rgba8();

        // Check watermark area at middle-left (around y=100)
        let modified = is_pixel_modified(rgba.get_pixel(30, 100), 30, 100);
        assert!(modified, "Middle-left area should be modified by watermark");
    }

    /// Test Center position - watermark should be at image center
    #[test]
    fn test_position_center() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF00FF".to_string(),
            position: WatermarkPosition::Center,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (200, 200));

        let rgba = decoded.to_rgba8();

        // Check watermark area at center
        let modified = is_pixel_modified(rgba.get_pixel(100, 100), 100, 100);
        assert!(modified, "Center area should be modified by watermark");
    }

    /// Test MiddleRight position - watermark should be at right edge center
    #[test]
    fn test_position_middle_right() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#00FFFF".to_string(),
            position: WatermarkPosition::MiddleRight,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (200, 200));

        let rgba = decoded.to_rgba8();

        // Check watermark area at middle-right
        let modified = is_pixel_modified(rgba.get_pixel(170, 100), 170, 100);
        assert!(modified, "Middle-right area should be modified by watermark");
    }

    /// Test BottomLeft position - watermark should be in bottom-left corner
    #[test]
    fn test_position_bottom_left() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FFA500".to_string(),
            position: WatermarkPosition::BottomLeft,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (200, 200));

        let rgba = decoded.to_rgba8();

        // Check watermark area at bottom-left
        let modified = is_pixel_modified(rgba.get_pixel(30, 170), 30, 170);
        assert!(modified, "Bottom-left area should be modified by watermark");
    }

    /// Test BottomCenter position - watermark should be at bottom edge center
    #[test]
    fn test_position_bottom_center() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#800080".to_string(),
            position: WatermarkPosition::BottomCenter,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (200, 200));

        let rgba = decoded.to_rgba8();

        // Check watermark area at bottom-center
        let modified = is_pixel_modified(rgba.get_pixel(100, 170), 100, 170);
        assert!(modified, "Bottom-center area should be modified by watermark");
    }

    /// Test BottomRight position - watermark should be in bottom-right corner
    #[test]
    fn test_position_bottom_right() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::BottomRight,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        assert_eq!(decoded.dimensions(), (200, 200));

        let rgba = decoded.to_rgba8();

        // Check watermark area at bottom-right
        let modified = is_pixel_modified(rgba.get_pixel(170, 170), 170, 170);
        assert!(modified, "Bottom-right area should be modified by watermark");
    }

    /// Test that different positions produce different output
    #[test]
    fn test_different_positions_produce_different_outputs() {
        let img = create_test_image();
        let input = encode_to_png(&img);

        let positions = [
            WatermarkPosition::TopLeft,
            WatermarkPosition::TopCenter,
            WatermarkPosition::TopRight,
            WatermarkPosition::MiddleLeft,
            WatermarkPosition::Center,
            WatermarkPosition::MiddleRight,
            WatermarkPosition::BottomLeft,
            WatermarkPosition::BottomCenter,
            WatermarkPosition::BottomRight,
        ];

        let mut outputs: Vec<Vec<u8>> = Vec::new();

        for position in &positions {
            let config = WatermarkConfig {
                text: "WM".to_string(),
                font_size: 24,
                color: "#FF0000".to_string(),
                position: *position,
                rotation: 0.0,
                font_weight: FontWeight::Normal,
            };

            let result = watermark::add_watermark(&input, &config);
            assert!(result.is_ok());
            outputs.push(result.unwrap());
        }

        // All outputs should be valid PNG
        for output in &outputs {
            assert!(image::load_from_memory(output).is_ok());
        }

        // Compare outputs - at least some should be different
        // TopLeft vs BottomRight should definitely differ
        assert_ne!(outputs[0], outputs[8]);
    }

    /// Test position calculation with different image sizes
    #[test]
    fn test_positions_with_different_image_sizes() {
        // Test with smaller image (100x100)
        let small_img = create_test_image_with_size(100, 100);
        let small_input = encode_to_png(&small_img);

        let config = WatermarkConfig {
            text: "WM".to_string(),
            font_size: 24,
            color: "#FF0000".to_string(),
            position: WatermarkPosition::Center,
            rotation: 0.0,
            font_weight: FontWeight::Normal,
        };

        let result = watermark::add_watermark(&small_input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();

        // Verify dimensions preserved
        assert_eq!(decoded.dimensions(), (100, 100));

        // Test with larger image (500x500)
        let large_img = create_test_image_with_size(500, 500);
        let large_input = encode_to_png(&large_img);

        let result2 = watermark::add_watermark(&large_input, &config);
        assert!(result2.is_ok());

        let output2 = result2.unwrap();
        let decoded2 = image::load_from_memory(&output2).unwrap();

        // Verify dimensions preserved
        assert_eq!(decoded2.dimensions(), (500, 500));
    }
}
