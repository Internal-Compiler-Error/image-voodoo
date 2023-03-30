use wasm_bindgen::Clamped;
use crate::canvas_image::CanvasImage;

use wasm_bindgen::prelude::*;
use web_sys::ImageData;

#[wasm_bindgen]
pub fn linear_transformation(image: ImageData, gain: f64, bias: f64) -> ImageData {
    let mut buffer = Vec::with_capacity((image.height() * image.width() * 4) as usize);

    let image = CanvasImage::new(image);

    for y in 0..image.height() {
        for x in 0..image.width() {
            let r = image.r(x, y).unwrap_or(0);
            let g = image.g(x, y).unwrap_or(0);
            let b = image.b(x, y).unwrap_or(0);
            let a = image.a(x, y).unwrap_or(0);

            let r = (r as f64  * gain + bias).clamp(0f64, 255f64) as u8;
            let g = (g as f64  * gain + bias).clamp(0f64, 255f64) as u8;
            let b = (b as f64  * gain + bias).clamp(0f64, 255f64) as u8;
            let a = (a as f64  * gain + bias).clamp(0f64, 255f64) as u8;

            buffer.push(r);
            buffer.push(g);
            buffer.push(b);
            buffer.push(a);
        }
    }

    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buffer), image.width(), image.height()).unwrap()
}

#[wasm_bindgen]
pub fn gamma_transformation(image: ImageData, gamma: f64) -> ImageData {
    let mut buffer = Vec::with_capacity((image.height() * image.width() * 4) as usize);
    let image = CanvasImage::new(image);

    for y in 0..image.height() {
        for x in 0..image.width() {
            let r = image.r(x, y).unwrap_or(0);
            let g = image.g(x, y).unwrap_or(0);
            let b = image.b(x, y).unwrap_or(0);
            let a = image.a(x, y).unwrap_or(0);


            let r = (255f64 * (r as f64 / 255f64).powf(gamma)) as u8;
            let g = (255f64 * (g as f64 / 255f64).powf(gamma)) as u8;
            let b = (255f64 * (b as f64 / 255f64).powf(gamma)) as u8;
            let a = (255f64 * (a as f64 / 255f64).powf(gamma)) as u8;

            buffer.push(r);
            buffer.push(g);
            buffer.push(b);
            buffer.push(a);
        }
    }

    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buffer), image.width(), image.height()).unwrap()
}