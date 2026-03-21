pub mod compress;
pub mod error;
pub mod io;
pub mod portrait;
pub mod types;
pub mod watermark;

pub use error::ImageError;
pub use types::*;

// WASM-specific exports
#[cfg(feature = "wasm")]
mod wasm_exports {
    use super::*;
    use wasm_bindgen::prelude::*;

    /// Initialize the WASM module
    #[wasm_bindgen(start)]
    pub fn init() {
        console_error_panic_hook::set_once();
    }

    /// Compress image with specified level and format
    #[wasm_bindgen]
    pub fn compress_image(input_data: &[u8], options: JsValue) -> Result<Vec<u8>, JsValue> {
        let config: CompressionConfig = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?;

        compress::compress(input_data, &config).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Add watermark to image
    #[wasm_bindgen]
    pub fn add_watermark(input_data: &[u8], options: JsValue) -> Result<Vec<u8>, JsValue> {
        let config: WatermarkConfig = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?;

        watermark::add_watermark(input_data, &config).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Create portrait photo with background color and size
    #[wasm_bindgen]
    pub fn create_portrait_photo(input_data: &[u8], options: JsValue) -> Result<Vec<u8>, JsValue> {
        let config: PortraitConfig = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?;

        portrait::create_portrait(input_data, &config).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[cfg(feature = "wasm")]
pub use wasm_exports::*;
