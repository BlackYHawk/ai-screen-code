use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{DynamicImage, Rgba, RgbaImage};

use crate::error::ImageError;
use crate::io;
use crate::types::WatermarkConfig;

/// Embedded font data (Monaco font)
const FONT_DATA: &[u8] = include_bytes!("../fonts/Monaco.ttf");

/// Add watermark to image - tiles watermark across entire image
pub fn add_watermark(data: &[u8], config: &WatermarkConfig) -> Result<Vec<u8>, ImageError> {
    let img = io::load_image(data)?;
    let (_, height) = io::get_dimensions(&img);

    // Parse hex color to RGBA
    let color = parse_hex_color(&config.color)?;

    // Create mutable RGBA image
    let mut result = img.to_rgba8();

    // Draw tiled watermark across entire image
    draw_tiled_watermark(
        &mut result,
        &config.text,
        config.font_size,
        color,
        config.rotation,
        config.spacing,
    )?;

    // Convert back to output format
    let output_img = DynamicImage::ImageRgba8(result);
    io::save_image(&output_img, crate::types::OutputFormat::Png)
}

/// Parse hex color string to RGBA
fn parse_hex_color(hex: &str) -> Result<Rgba<u8>, ImageError> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 && hex.len() != 8 {
        return Err(ImageError::InvalidParameter(
            "Invalid hex color format".to_string(),
        ));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| ImageError::InvalidParameter("Invalid red component".to_string()))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| ImageError::InvalidParameter("Invalid green component".to_string()))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| ImageError::InvalidParameter("Invalid blue component".to_string()))?;
    let a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
    } else {
        255
    };

    Ok(Rgba([r, g, b, a]))
}

/// Draw tiled watermark across the entire image
fn draw_tiled_watermark(
    img: &mut RgbaImage,
    text: &str,
    font_size: u32,
    color: Rgba<u8>,
    rotation: f32,
    spacing: u32,
) -> Result<(), ImageError> {
    // Load font
    let font = FontRef::try_from_slice(FONT_DATA)
        .map_err(|e| ImageError::ProcessingError(format!("Failed to load font: {}", e)))?;

    let scale = PxScale::from(font_size as f32);
    let font_for_outline = font.clone();
    let scaled_font = font.into_scaled(scale);
    let font_for_draw = font_for_outline.clone();

    // Calculate single text dimensions
    let mut text_width: f32 = 0.0;
    let mut text_height: f32 = 0.0;

    for c in text.chars() {
        let glyph_id = scaled_font.glyph_id(c);
        let h_advance = scaled_font.h_advance(glyph_id);
        let v_advance = scaled_font.v_advance(glyph_id);
        text_width += h_advance;
        text_height = text_height.max(v_advance);
    }

    // Calculate tile size (text size + spacing)
    let tile_width = (text_width + spacing as f32).max(1.0);
    let tile_height = (text_height + spacing as f32).max(1.0);

    // Get image dimensions
    let img_width = img.width();
    let img_height = img.height();

    // Calculate number of tiles needed (with some overlap for rotation)
    let rotation_radians = rotation.to_radians();
    let rotated_offset = if rotation != 0.0 {
        ((text_width * rotation_radians.cos().abs())
            + (text_height * rotation_radians.sin().abs())) as u32
    } else {
        0
    };

    let tiles_x = ((img_width as f32 / tile_width) as u32) + 3;
    let tiles_y = ((img_height as f32 / tile_height) as u32) + 3;

    // Draw tiled watermarks - start from -1 to ensure full coverage
    for tile_y in 0..tiles_y {
        for tile_x in 0..tiles_x {
            let base_x = ((tile_x as i32 - 1) as f32 * tile_width) as i32;
            let base_y = ((tile_y as i32 - 1) as f32 * tile_height) as i32;

            // Draw the watermark text at this position with rotation
            draw_rotated_text(
                img,
                text,
                font_for_draw.clone(),
                scale,
                color,
                base_x,
                base_y,
                rotation,
            )?;
        }
    }

    Ok(())
}

/// Draw a single text at position with rotation
fn draw_rotated_text(
    img: &mut RgbaImage,
    text: &str,
    font: FontRef,
    scale: PxScale,
    color: Rgba<u8>,
    start_x: i32,
    start_y: i32,
    rotation: f32,
) -> Result<(), ImageError> {
    let img_width = img.width() as i32;
    let img_height = img.height() as i32;
    let scaled_font = font.clone().into_scaled(scale);

    // If no rotation, draw normally
    if rotation == 0.0 {
        let mut x = start_x as f32;
        for c in text.chars() {
            let glyph_id = scaled_font.glyph_id(c);
            let h_advance = scaled_font.h_advance(glyph_id);
            let glyph = glyph_id.with_scale(scale);

            if let Some(outline) = font.outline_glyph(glyph) {
                outline.draw(|px, py, coverage| {
                    if coverage > 0.0 {
                        let px = (px as f32 + x) as i32;
                        let py = (py as f32 + start_y as f32) as i32;
                        if px >= 0 && py >= 0 && px < img_width && py < img_height {
                            let alpha = (color.0[3] as f32 * coverage).min(255.0) as u8;
                            let pixel = img.get_pixel_mut(px as u32, py as u32);
                            let src_alpha = alpha as f32 / 255.0;
                            let dst_alpha = 1.0 - src_alpha;
                            pixel.0[0] =
                                ((color.0[0] as f32 * src_alpha) + (pixel.0[0] as f32 * dst_alpha))
                                    as u8;
                            pixel.0[1] =
                                ((color.0[1] as f32 * src_alpha) + (pixel.0[1] as f32 * dst_alpha))
                                    as u8;
                            pixel.0[2] =
                                ((color.0[2] as f32 * src_alpha) + (pixel.0[2] as f32 * dst_alpha))
                                    as u8;
                            pixel.0[3] =
                                ((color.0[3] as f32 * src_alpha) + (pixel.0[3] as f32 * dst_alpha))
                                    .min(255.0) as u8;
                        }
                    }
                });
            }
            x += h_advance;
        }
    } else {
        // With rotation - draw each character rotated around its center
        let mut x = start_x as f32;
        let angle = rotation.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let font_height = scaled_font.height();

        for c in text.chars() {
            let glyph_id = scaled_font.glyph_id(c);
            let h_advance = scaled_font.h_advance(glyph_id);
            let glyph = glyph_id.with_scale(scale);

            if let Some(outline) = font.outline_glyph(glyph) {
                // Calculate character center for rotation
                let char_center_x = x + (h_advance / 2.0);
                let char_center_y = start_y as f32 + (font_height / 2.0);

                outline.draw(|px, py, coverage| {
                    if coverage > 0.0 {
                        // Translate to character center
                        let dx = px as f32 - char_center_x;
                        let dy = py as f32 - char_center_y;

                        // Rotate
                        let rotated_x = char_center_x + dx * cos_a - dy * sin_a;
                        let rotated_y = char_center_y + dx * sin_a + dy * cos_a;

                        let px = rotated_x as i32;
                        let py = rotated_y as i32;

                        if px >= 0 && py >= 0 && px < img_width && py < img_height {
                            let alpha = (color.0[3] as f32 * coverage).min(255.0) as u8;
                            let pixel = img.get_pixel_mut(px as u32, py as u32);
                            let src_alpha = alpha as f32 / 255.0;
                            let dst_alpha = 1.0 - src_alpha;
                            pixel.0[0] =
                                ((color.0[0] as f32 * src_alpha) + (pixel.0[0] as f32 * dst_alpha))
                                    as u8;
                            pixel.0[1] =
                                ((color.0[1] as f32 * src_alpha) + (pixel.0[1] as f32 * dst_alpha))
                                    as u8;
                            pixel.0[2] =
                                ((color.0[2] as f32 * src_alpha) + (pixel.0[2] as f32 * dst_alpha))
                                    as u8;
                            pixel.0[3] =
                                ((color.0[3] as f32 * src_alpha) + (pixel.0[3] as f32 * dst_alpha))
                                    .min(255.0) as u8;
                        }
                    }
                });
            }
            x += h_advance;
        }
    }

    Ok(())
}
