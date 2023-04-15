use crate::canvas_image::CanvasImage;
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

/// Convert sRGB to linear RGB
pub trait Linearize {
    fn linearize(&self) -> f64;
}

impl Linearize for f64 {
    // Plagiarized from wikipedia
    fn linearize(&self) -> f64 {
        if *self <= 0.04045 {
            *self / 12.92
        } else {
            ((self + 0.055) / 1.055).powf(2.4)
        }
    }
}

/// Convert linear RGB to luminance defined in CIE 1931
// Plagiarized from wikipedia
pub fn to_luminance(linear_r: f64, linear_g: f64, linear_b: f64) -> f64 {
    0.2126 * linear_r + 0.7152 * linear_g + 0.0722 * linear_b
}

// Plagiarized from wikipedia
pub fn to_srgb(linear: f64) -> f64 {
    if linear <= 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}

#[wasm_bindgen]
pub fn faster_greyscale(image: ImageData) -> Vec<u8> {
    let mut image = CanvasImage::from_image_data(image);
    image.convert_to_greyscale();
    image.into()
}
