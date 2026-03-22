use serde::{Deserialize, Serialize};

/// Compression level for image compression
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CompressionLevel {
    /// Light - quality priority
    Light,
    /// Normal - balanced
    Normal,
    /// Strong - size priority
    Strong,
    /// Extreme - smallest size
    Extreme,
}

impl Default for CompressionLevel {
    fn default() -> Self {
        CompressionLevel::Normal
    }
}

/// Output format for processed images
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Jpeg,
    Png,
    WebP,
    Bmp,
    Gif,
    Tiff,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Png
    }
}

/// Configuration for image compression
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressionConfig {
    pub level: CompressionLevel,
    pub format: OutputFormat,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        CompressionConfig {
            level: CompressionLevel::default(),
            format: OutputFormat::default(),
        }
    }
}

/// Position for watermark placement
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum WatermarkPosition {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    Center,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Default for WatermarkPosition {
    fn default() -> Self {
        WatermarkPosition::BottomRight
    }
}

/// Font weight for watermark text
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FontWeight {
    Normal,
    Bold,
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

/// Configuration for watermark
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatermarkConfig {
    pub text: String,
    pub font_size: u32,
    pub color: String, // hex format like "#FF0000"
    pub position: WatermarkPosition,
    pub rotation: f32, // 0-360 degrees, applies to tiled watermark
    pub font_weight: FontWeight,
    pub spacing: u32, // spacing between tiled watermarks in pixels
}

impl Default for WatermarkConfig {
    fn default() -> Self {
        WatermarkConfig {
            text: String::new(),
            font_size: 24,
            color: "#000000".to_string(),
            position: WatermarkPosition::default(),
            rotation: 0.0,
            font_weight: FontWeight::default(),
            spacing: 100, // default spacing
        }
    }
}

/// Background color for portrait photos
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BackgroundColor {
    White,
    Red,
    Blue,
}

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor::White
    }
}

/// Photo size specifications (in mm)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PhotoSize {
    OneInch,      // 25×35mm
    OneInchSmall, // 22×32mm
    OneInchLarge, // 33×48mm
    TwoInch,      // 35×49mm
    TwoInchSmall, // 35×45mm
    TwoInchLarge, // 35×53mm
    ThreeInch,    // 58×84mm
    FourInch,     // 76×102mm
    FiveInch,     // 89×127mm
}

impl PhotoSize {
    /// Returns (width_mm, height_mm)
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            PhotoSize::OneInch => (25, 35),
            PhotoSize::OneInchSmall => (22, 32),
            PhotoSize::OneInchLarge => (33, 48),
            PhotoSize::TwoInch => (35, 49),
            PhotoSize::TwoInchSmall => (35, 45),
            PhotoSize::TwoInchLarge => (35, 53),
            PhotoSize::ThreeInch => (58, 84),
            PhotoSize::FourInch => (76, 102),
            PhotoSize::FiveInch => (89, 127),
        }
    }

    /// Convert mm to pixels at 300 DPI
    pub fn to_pixels(&self) -> (u32, u32) {
        const DPI: f32 = 300.0;
        const MM_PER_INCH: f32 = 25.4;

        let (w_mm, h_mm) = self.dimensions();
        let w_px = (w_mm as f32 / MM_PER_INCH * DPI).round() as u32;
        let h_px = (h_mm as f32 / MM_PER_INCH * DPI).round() as u32;
        (w_px, h_px)
    }
}

impl Default for PhotoSize {
    fn default() -> Self {
        PhotoSize::TwoInch
    }
}

/// Configuration for portrait photo generation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortraitConfig {
    pub background: BackgroundColor,
    pub size: PhotoSize,
}

impl Default for PortraitConfig {
    fn default() -> Self {
        PortraitConfig {
            background: BackgroundColor::default(),
            size: PhotoSize::default(),
        }
    }
}
