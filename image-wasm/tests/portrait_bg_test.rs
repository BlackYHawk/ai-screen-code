//! Tests for portrait background color replacement (T038)
//!
//! Tests 3 background colors:
//! - White (白底)
//! - Red (红底)
//! - Blue (蓝底)

use image::{ImageBuffer, Rgba, ImageEncoder};
use image_wasm::portrait::create_portrait;
use image_wasm::{BackgroundColor, PhotoSize, PortraitConfig};

/// Create a test portrait image with white background
fn create_white_bg_portrait() -> Vec<u8> {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(200, 300, |x, y| {
        let center_x = 100;
        let center_y = 150;

        // Default: white background
        let mut pixel = Rgba([255, 255, 255, 255]);

        // Add person figure
        // Head
        let head_radius = 40;
        let dx = x as i32 - center_x as i32;
        let dy = y as i32 - (center_y - 50) as i32;
        if (dx * dx + dy * dy) as u32 <= head_radius * head_radius {
            pixel = Rgba([200, 150, 100, 255]); // Skin tone
        }

        // Body
        let body_width = 60;
        let body_height = 100;
        let body_top = center_y;
        let dy_body = y as i32 - body_top as i32;
        let dx_body = x as i32 - center_x as i32;
        if dy_body >= 0
            && dy_body < body_height as i32
            && ((dx_body * dx_body) as f32 / (body_width * body_width) as f32
                + (dy_body * dy_body) as f32 / (body_height * body_height) as f32)
                <= 1.0
        {
            pixel = Rgba([50, 100, 150, 255]); // Blue shirt
        }

        pixel
    });

    encode_to_png(&img)
}

/// Create a test portrait image with light gray background
fn create_light_gray_bg_portrait() -> Vec<u8> {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(200, 300, |x, y| {
        let center_x = 100;
        let center_y = 150;

        // Light gray background (should be detected as background)
        let mut pixel = Rgba([220, 220, 220, 255]);

        // Add person figure
        let head_radius = 40;
        let dx = x as i32 - center_x as i32;
        let dy = y as i32 - (center_y - 50) as i32;
        if (dx * dx + dy * dy) as u32 <= head_radius * head_radius {
            pixel = Rgba([200, 150, 100, 255]); // Skin tone
        }

        let body_width = 60;
        let body_height = 100;
        let body_top = center_y;
        let dy_body = y as i32 - body_top as i32;
        let dx_body = x as i32 - center_x as i32;
        if dy_body >= 0
            && dy_body < body_height as i32
            && ((dx_body * dx_body) as f32 / (body_width * body_width) as f32
                + (dy_body * dy_body) as f32 / (body_height * body_height) as f32)
                <= 1.0
        {
            pixel = Rgba([50, 100, 150, 255]); // Blue shirt
        }

        pixel
    });

    encode_to_png(&img)
}

/// Encode image to PNG bytes
fn encode_to_png(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<u8> {
    let mut buffer = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
        encoder
            .write_image(
                img.as_raw(),
                img.width(),
                img.height(),
                image::ExtendedColorType::Rgba8,
            )
            .unwrap();
    }
    buffer
}

mod background_color_replacement {
    use super::*;

    #[test]
    fn test_replace_to_white() {
        let input = create_white_bg_portrait();
        let config = PortraitConfig {
            background: BackgroundColor::White,
            size: PhotoSize::TwoInch,
        };

        let result = create_portrait(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        let rgba = decoded.to_rgba8();

        // Check corners are white
        let corner = rgba.get_pixel(0, 0);
        assert!(
            corner[0] > 250 && corner[1] > 250 && corner[2] > 250,
            "Corner should be white: {:?}",
            corner
        );
    }

    #[test]
    fn test_replace_to_red() {
        let input = create_white_bg_portrait();
        let config = PortraitConfig {
            background: BackgroundColor::Red,
            size: PhotoSize::TwoInch,
        };

        let result = create_portrait(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        let rgba = decoded.to_rgba8();

        // Check corners are red
        let corner = rgba.get_pixel(0, 0);
        // Red background color: RGB(220, 53, 69)
        assert!(
            corner[0] > 200 && corner[1] < 100 && corner[2] < 100,
            "Corner should be red: {:?}",
            corner
        );
    }

    #[test]
    fn test_replace_to_blue() {
        let input = create_white_bg_portrait();
        let config = PortraitConfig {
            background: BackgroundColor::Blue,
            size: PhotoSize::TwoInch,
        };

        let result = create_portrait(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        let rgba = decoded.to_rgba8();

        // Check corners are blue
        let corner = rgba.get_pixel(0, 0);
        // Blue background color: RGB(0, 123, 255)
        assert!(
            corner[0] < 50 && corner[1] > 100 && corner[2] > 200,
            "Corner should be blue: {:?}",
            corner
        );
    }

    #[test]
    fn test_all_background_colors() {
        let input = create_white_bg_portrait();
        let colors = [
            BackgroundColor::White,
            BackgroundColor::Red,
            BackgroundColor::Blue,
        ];

        for color in colors {
            let config = PortraitConfig {
                background: color,
                size: PhotoSize::TwoInch,
            };

            let result = create_portrait(&input, &config);
            assert!(result.is_ok(), "Failed for background color {:?}", color);

            let output = result.unwrap();
            assert!(!output.is_empty(), "Output should not be empty for {:?}", color);
        }
    }
}

mod background_detection {
    use super::*;

    #[test]
    fn test_detect_white_background() {
        let input = create_white_bg_portrait();
        let config = PortraitConfig {
            background: BackgroundColor::Blue,
            size: PhotoSize::TwoInch,
        };

        let result = create_portrait(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        let rgba = decoded.to_rgba8();

        // With simple threshold detection, corners should be blue
        // Check corners are blue (not white)
        let corner = rgba.get_pixel(0, 0);
        assert!(
            corner[2] > 200,
            "Corner should be blue: {:?}",
            corner
        );
    }

    #[test]
    fn test_detect_light_gray_background() {
        let input = create_light_gray_bg_portrait();
        let config = PortraitConfig {
            background: BackgroundColor::Red,
            size: PhotoSize::TwoInch,
        };

        let result = create_portrait(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        let rgba = decoded.to_rgba8();

        // Corners should be red now
        let corner = rgba.get_pixel(0, 0);
        assert!(
            corner[0] > 200 && corner[1] < 100 && corner[2] < 100,
            "Corner should be red: {:?}",
            corner
        );
    }
}

mod background_preserves_foreground {
    use super::*;

    #[test]
    fn test_preserves_person_when_changing_bg() {
        let input = create_white_bg_portrait();

        // Process with different backgrounds
        let configs = [
            PortraitConfig {
                background: BackgroundColor::White,
                size: PhotoSize::TwoInch,
            },
            PortraitConfig {
                background: BackgroundColor::Red,
                size: PhotoSize::TwoInch,
            },
            PortraitConfig {
                background: BackgroundColor::Blue,
                size: PhotoSize::TwoInch,
            },
        ];

        let mut outputs = Vec::new();
        for config in &configs {
            let result = create_portrait(&input, config).unwrap();
            outputs.push(image::load_from_memory(&result).unwrap());
        }

        // Verify that background colors are different in corners
        let rgba_white = outputs[0].to_rgba8();
        let rgba_red = outputs[1].to_rgba8();
        let rgba_blue = outputs[2].to_rgba8();

        let corner_white = rgba_white.get_pixel(0, 0);
        let corner_red = rgba_red.get_pixel(0, 0);
        let corner_blue = rgba_blue.get_pixel(0, 0);

        // White background corners should be white
        assert!(
            corner_white[0] > 250 && corner_white[1] > 250 && corner_white[2] > 250,
            "White bg corner should be white: {:?}",
            corner_white
        );

        // Red background corners should be red
        assert!(
            corner_red[0] > 200 && corner_red[1] < 100 && corner_red[2] < 100,
            "Red bg corner should be red: {:?}",
            corner_red
        );

        // Blue background corners should be blue
        assert!(
            corner_blue[0] < 50 && corner_blue[1] > 100 && corner_blue[2] > 200,
            "Blue bg corner should be blue: {:?}",
            corner_blue
        );

        // Different background outputs should have different corner colors
        // Compare red and blue corners (they should be clearly different)
        let corner_diff_red_blue = (corner_red[0] as i32 - corner_blue[0] as i32).abs()
            + (corner_red[2] as i32 - corner_blue[2] as i32).abs();
        assert!(
            corner_diff_red_blue > 100,
            "Corners should differ between red and blue bg: {}",
            corner_diff_red_blue
        );
    }
}

mod background_color_enum {
    use super::*;

    #[test]
    fn test_background_color_default() {
        let bg = BackgroundColor::default();
        assert_eq!(bg, BackgroundColor::White);
    }

    #[test]
    fn test_all_background_colors_exist() {
        // Verify all three colors are available
        let _ = BackgroundColor::White;
        let _ = BackgroundColor::Red;
        let _ = BackgroundColor::Blue;
    }

    #[test]
    fn test_portrait_config_default() {
        let config = PortraitConfig::default();
        assert_eq!(config.background, BackgroundColor::default());
        assert_eq!(config.size, PhotoSize::default());
    }
}
