mod css_filter;

pub use css_filter::{FilterResult, css_filter};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub fn color_to_filter(color: &str) -> Result<String, String> {
    let color = csscolorparser::parse(color)
        .map_err(|e| e.to_string())?;
    // r, g, b are between 0-1, multiply by 255, 256 overflows
    let (r, g, b) = (
        (color.r * 255f32).round() as u8,
        (color.g * 255f32).round() as u8,
        (color.b * 255f32).round() as u8
    );
    Ok(css_filter(r, g, b))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_color_to_filter(color: &str) -> Result<String, JsError> {
    color_to_filter(color).map_err(|e| JsError::new(&e))
}
