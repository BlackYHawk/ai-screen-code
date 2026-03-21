use crate::error::ImageError;
use crate::io;
use crate::types::{BackgroundColor, PortraitConfig};
use image::{DynamicImage, Rgba, RgbaImage};

/// Create portrait photo with background color and specified size
pub fn create_portrait(data: &[u8], config: &PortraitConfig) -> Result<Vec<u8>, ImageError> {
    let img = io::load_image(data)?;
    let (target_width, target_height) = config.size.to_pixels();

    // Replace background color
    let mut processed = replace_background(&img, config.background)?;

    // Resize to target dimensions
    processed = processed.resize_exact(
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
    );

    // Save as PNG
    io::save_image(&processed, crate::types::OutputFormat::Png)
}

/// Replace image background with specified color
fn replace_background(
    img: &DynamicImage,
    bg_color: BackgroundColor,
) -> Result<DynamicImage, ImageError> {
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    let target = match bg_color {
        BackgroundColor::White => Rgba([255, 255, 255, 255]),
        BackgroundColor::Red => Rgba([220, 53, 69, 255]),
        BackgroundColor::Blue => Rgba([0, 123, 255, 255]),
    };

    let mut result = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = rgba.get_pixel(x, y);
            let is_background = is_likely_background(pixel);

            if is_background {
                result.put_pixel(x, y, target);
            } else {
                result.put_pixel(x, y, *pixel);
            }
        }
    }

    Ok(DynamicImage::ImageRgba8(result))
}

/// Check if a pixel is likely a light background
fn is_likely_background(pixel: &Rgba<u8>) -> bool {
    let r = pixel.0[0] as f32;
    let g = pixel.0[1] as f32;
    let b = pixel.0[2] as f32;
    let a = pixel.0[3];

    if a < 128 {
        return false;
    }

    let brightness = (r + g + b) / 3.0;
    let is_white = r > 240.0 && g > 240.0 && b > 240.0;
    let is_light_gray = r > 200.0 && g > 200.0 && b > 200.0 && !is_white;

    brightness > 200.0 || is_white || is_light_gray
}
