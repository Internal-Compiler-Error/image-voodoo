use wasm_bindgen::Clamped;
use crate::canvas_image::CanvasImage;

use wasm_bindgen::prelude::*;
use web_sys::ImageData;

#[wasm_bindgen]
pub fn linear_transformation(image: ImageData, gain: f64, bias: f64) -> ImageData {
    let image = CanvasImage::new(image);

    let transformed = image.rgba_iter()
        .flat_map(|(r, g, b, a)| {
            let r = (r as f64 * gain + bias).clamp(0f64, 255f64) as u8;
            let g = (g as f64 * gain + bias).clamp(0f64, 255f64) as u8;
            let b = (b as f64 * gain + bias).clamp(0f64, 255f64) as u8;
            let a = (a as f64 * gain + bias).clamp(0f64, 255f64) as u8;

            [r, g, b, a]
        });
    let buffer = Vec::from_iter(transformed);
    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buffer), image.horizontal_size(), image.vertical_size()).unwrap()
}

#[wasm_bindgen]
pub fn gamma_transformation(image: ImageData, gamma: f64) -> ImageData {
    let image = CanvasImage::new(image);

    let rgba = image.rgba_iter()
        .flat_map(|(r, g, b, a)| {
            let r = (255f64 * (r as f64 / 255f64).powf(gamma)) as u8;
            let g = (255f64 * (g as f64 / 255f64).powf(gamma)) as u8;
            let b = (255f64 * (b as f64 / 255f64).powf(gamma)) as u8;
            let a = (255f64 * (a as f64 / 255f64).powf(gamma)) as u8;

            [r, g, b, a]
        });
    let buffer = Vec::from_iter(rgba);

    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buffer), image.horizontal_size(), image.vertical_size()).unwrap()
}