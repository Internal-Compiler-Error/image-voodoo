use itertools::iproduct;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::ImageData;
use crate::canvas_image::CanvasImage;


/// Flips the image horizontally, along the x-axis.
#[wasm_bindgen]
pub fn flip_along_x_axis(image: ImageData) -> ImageData {
    let image = CanvasImage::new(image);

    // flip along the x-axis
    let rgba = iproduct!(0..image.height(), 0..image.width())
        .map(|(y, x)| {
            let flipped_y = image.height() - y - 1;

            (flipped_y, x)
        })
        .flat_map(|(y, x)| {
            let r = image.r(x, y).unwrap();
            let g = image.g(x, y).unwrap();
            let b = image.b(x, y).unwrap();
            let a = image.a(x, y).unwrap();

            [r, g, b, a]
        });

    let buffer = Vec::from_iter(rgba);

    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buffer), image.width(), image.height()).unwrap()
}

/// Flips the image horizontally, along the x-axis.
#[wasm_bindgen]
pub fn flip_along_y_axis(image: ImageData) -> ImageData {
    let image = CanvasImage::new(image);

    // flip along the x-axis
    let rgba = iproduct!(0..image.height(), 0..image.width())
        .map(|(y, x)| {
            let flipped_x = image.width() - x - 1;
            (y, flipped_x)
        })
        .flat_map(|(y, x)| {
            let r = image.r(x, y).unwrap();
            let g = image.g(x, y).unwrap();
            let b = image.b(x, y).unwrap();
            let a = image.a(x, y).unwrap();

            [r, g, b, a]
        });

    let buffer = Vec::from_iter(rgba);

    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buffer), image.width(), image.height()).unwrap()
}