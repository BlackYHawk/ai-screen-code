//! Tests for portrait photo sizes (T037)
//!
//! Tests 9 photo size specifications at 300 DPI:
//! - OneInch (一寸): 25×35mm
//! - OneInchSmall (小一寸): 22×32mm
//! - OneInchLarge (大一寸): 33×48mm
//! - TwoInch (二寸): 35×49mm
//! - TwoInchSmall (小二寸): 35×45mm
//! - TwoInchLarge (大二寸): 35×53mm
//! - ThreeInch (三寸): 58×84mm
//! - FourInch (四寸): 76×102mm
//! - FiveInch (五寸): 89×127mm

use image::{GenericImageView, ImageBuffer, Rgba, ImageEncoder};
use image_wasm::portrait::create_portrait;
use image_wasm::{BackgroundColor, PhotoSize, PortraitConfig};

/// Create a simple test portrait image (200x300 with white background and a person-like figure)
fn create_test_portrait_image() -> Vec<u8> {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(200, 300, |x, y| {
        // Create white background
        let mut pixel = Rgba([255, 255, 255, 255]);

        // Add a simple person-like figure in the center
        let center_x = 100;
        let center_y = 150;

        // Head (circle)
        let head_radius = 40;
        let dx = x as i32 - center_x as i32;
        let dy = y as i32 - (center_y - 50) as i32;
        if (dx * dx + dy * dy) as u32 <= head_radius * head_radius {
            pixel = Rgba([200, 150, 100, 255]); // Skin tone
        }

        // Body (oval)
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

    // Encode to PNG bytes
    let mut buffer = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
        encoder
            .write_image(img.as_raw(), 200, 300, image::ExtendedColorType::Rgba8)
            .unwrap();
    }
    buffer
}

mod photo_size_dimensions {
    use super::*;

    /// Verify mm to pixels conversion at 300 DPI
    /// 1 inch = 25.4mm, so 300 pixels per inch
    fn mm_to_pixels(mm: f32) -> u32 {
        (mm / 25.4 * 300.0).round() as u32
    }

    #[test]
    fn test_one_inch_dimensions() {
        let (w_mm, h_mm) = PhotoSize::OneInch.dimensions();
        assert_eq!(w_mm, 25);
        assert_eq!(h_mm, 35);

        let (w_px, h_px) = PhotoSize::OneInch.to_pixels();
        assert_eq!(w_px, mm_to_pixels(25.0));
        assert_eq!(h_px, mm_to_pixels(35.0));
    }

    #[test]
    fn test_one_inch_small_dimensions() {
        let (w_mm, h_mm) = PhotoSize::OneInchSmall.dimensions();
        assert_eq!(w_mm, 22);
        assert_eq!(h_mm, 32);

        let (w_px, h_px) = PhotoSize::OneInchSmall.to_pixels();
        assert_eq!(w_px, mm_to_pixels(22.0));
        assert_eq!(h_px, mm_to_pixels(32.0));
    }

    #[test]
    fn test_one_inch_large_dimensions() {
        let (w_mm, h_mm) = PhotoSize::OneInchLarge.dimensions();
        assert_eq!(w_mm, 33);
        assert_eq!(h_mm, 48);

        let (w_px, h_px) = PhotoSize::OneInchLarge.to_pixels();
        assert_eq!(w_px, mm_to_pixels(33.0));
        assert_eq!(h_px, mm_to_pixels(48.0));
    }

    #[test]
    fn test_two_inch_dimensions() {
        let (w_mm, h_mm) = PhotoSize::TwoInch.dimensions();
        assert_eq!(w_mm, 35);
        assert_eq!(h_mm, 49);

        let (w_px, h_px) = PhotoSize::TwoInch.to_pixels();
        assert_eq!(w_px, mm_to_pixels(35.0));
        assert_eq!(h_px, mm_to_pixels(49.0));
    }

    #[test]
    fn test_two_inch_small_dimensions() {
        let (w_mm, h_mm) = PhotoSize::TwoInchSmall.dimensions();
        assert_eq!(w_mm, 35);
        assert_eq!(h_mm, 45);

        let (w_px, h_px) = PhotoSize::TwoInchSmall.to_pixels();
        assert_eq!(w_px, mm_to_pixels(35.0));
        assert_eq!(h_px, mm_to_pixels(45.0));
    }

    #[test]
    fn test_two_inch_large_dimensions() {
        let (w_mm, h_mm) = PhotoSize::TwoInchLarge.dimensions();
        assert_eq!(w_mm, 35);
        assert_eq!(h_mm, 53);

        let (w_px, h_px) = PhotoSize::TwoInchLarge.to_pixels();
        assert_eq!(w_px, mm_to_pixels(35.0));
        assert_eq!(h_px, mm_to_pixels(53.0));
    }

    #[test]
    fn test_three_inch_dimensions() {
        let (w_mm, h_mm) = PhotoSize::ThreeInch.dimensions();
        assert_eq!(w_mm, 58);
        assert_eq!(h_mm, 84);

        let (w_px, h_px) = PhotoSize::ThreeInch.to_pixels();
        assert_eq!(w_px, mm_to_pixels(58.0));
        assert_eq!(h_px, mm_to_pixels(84.0));
    }

    #[test]
    fn test_four_inch_dimensions() {
        let (w_mm, h_mm) = PhotoSize::FourInch.dimensions();
        assert_eq!(w_mm, 76);
        assert_eq!(h_mm, 102);

        let (w_px, h_px) = PhotoSize::FourInch.to_pixels();
        assert_eq!(w_px, mm_to_pixels(76.0));
        assert_eq!(h_px, mm_to_pixels(102.0));
    }

    #[test]
    fn test_five_inch_dimensions() {
        let (w_mm, h_mm) = PhotoSize::FiveInch.dimensions();
        assert_eq!(w_mm, 89);
        assert_eq!(h_mm, 127);

        let (w_px, h_px) = PhotoSize::FiveInch.to_pixels();
        assert_eq!(w_px, mm_to_pixels(89.0));
        assert_eq!(h_px, mm_to_pixels(127.0));
    }
}

mod portrait_resize {
    use super::*;

    #[test]
    fn test_resize_to_one_inch() {
        let input = create_test_portrait_image();
        let config = PortraitConfig {
            background: BackgroundColor::White,
            size: PhotoSize::OneInch,
        };

        let result = create_portrait(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        let (w, h) = decoded.dimensions();
        let (target_w, target_h) = PhotoSize::OneInch.to_pixels();

        assert_eq!(w, target_w, "Width should match target");
        assert_eq!(h, target_h, "Height should match");
    }

    #[test]
    fn test_resize_to_two_inch() {
        let input = create_test_portrait_image();
        let config = PortraitConfig {
            background: BackgroundColor::White,
            size: PhotoSize::TwoInch,
        };

        let result = create_portrait(&input, &config);
        assert!(result.is_ok());

        let output = result.unwrap();
        let decoded = image::load_from_memory(&output).unwrap();
        let (w, h) = decoded.dimensions();
        let (target_w, target_h) = PhotoSize::TwoInch.to_pixels();

        assert_eq!(w, target_w);
        assert_eq!(h, target_h);
    }

    #[test]
    fn test_resize_to_all_sizes() {
        let input = create_test_portrait_image();
        let sizes = [
            PhotoSize::OneInch,
            PhotoSize::OneInchSmall,
            PhotoSize::OneInchLarge,
            PhotoSize::TwoInch,
            PhotoSize::TwoInchSmall,
            PhotoSize::TwoInchLarge,
            PhotoSize::ThreeInch,
            PhotoSize::FourInch,
            PhotoSize::FiveInch,
        ];

        for size in sizes {
            let config = PortraitConfig {
                background: BackgroundColor::White,
                size,
            };

            let result = create_portrait(&input, &config);
            assert!(result.is_ok(), "Failed for size {:?}", size);

            let output = result.unwrap();
            let decoded = image::load_from_memory(&output).unwrap();
            let (w, h) = decoded.dimensions();
            let (target_w, target_h) = size.to_pixels();

            assert_eq!(w, target_w, "Width mismatch for {:?}", size);
            assert_eq!(h, target_h, "Height mismatch for {:?}", size);
        }
    }

    #[test]
    fn test_default_size() {
        let config = PortraitConfig::default();
        let (w, h) = config.size.to_pixels();
        // Default is TwoInch
        let (expected_w, expected_h) = PhotoSize::TwoInch.to_pixels();
        assert_eq!(w, expected_w);
        assert_eq!(h, expected_h);
    }
}

mod dpi_standard {
    use super::*;

    /// Verify 300 DPI is the standard
    #[test]
    fn test_300_dpi_standard() {
        // At 300 DPI:
        // - 1 inch = 25.4mm = 300 pixels
        // - 25mm ≈ 295 pixels
        // - 35mm ≈ 413 pixels

        let (w, h) = PhotoSize::OneInch.to_pixels();

        // 25mm * 300 / 25.4 ≈ 295
        // 35mm * 300 / 25.4 ≈ 413
        assert_eq!(w, 295); // 25 * 300 / 25.4 ≈ 295
        assert_eq!(h, 413); // 35 * 300 / 25.4 ≈ 413
    }

    #[test]
    fn test_two_inch_300_dpi() {
        // 35mm * 300 / 25.4 ≈ 413
        // 49mm * 300 / 25.4 ≈ 579

        let (w, h) = PhotoSize::TwoInch.to_pixels();
        assert_eq!(w, 413);
        assert_eq!(h, 579);
    }
}
