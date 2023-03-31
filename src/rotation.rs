use crate::canvas_image::CanvasImage;
use itertools::iproduct;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::ImageData;

/// Rotate the image by radian, enlarge the image to fit the rotated image, empty spaces are filled
/// using bilinear interpolation.
pub fn rotate_rad(image: &CanvasImage, radian: f64) -> CanvasImage {
    // first reduce the radian to the range of [0, 2 * PI)
    let width = image.width() as f64;
    let height = image.height() as f64;

    let (new_width, new_height) = (
        (width * radian.cos() + height * radian.sin()).ceil() as u32 + 1,
        (width * radian.sin() + height * radian.cos()).ceil() as u32 + 1,
    );

    let (cx, cy) = (image.width() as f64 / 2f64, image.height() as f64 / 2f64);
    let (ox, oy) = (image.width() as f64 / 2f64, image.height() as f64 / 2f64);

    let rgba = iproduct!(0..new_height, 0..new_width).flat_map(|(y, x)| {
        let x = x as f64 - cx;
        let y = y as f64 - cy;

        let x = x * radian.cos() + y * radian.sin();
        let y = x * radian.sin() + y * radian.cos();

        let x = x + cx;
        let y = y + cy;

        let x = x + ox;
        let y = y + oy;

        let pixel = image.rgba(x as u32, y as u32).unwrap_or((128, 64, 32, 0));
        [pixel.0, pixel.1, pixel.2, pixel.3]
    });

    let buffer = Vec::from_iter(rgba);
    CanvasImage::from_vec_with_size(buffer, new_width, new_height)
}

/// Rotate the image by degree, enlarge the image to fit the rotated image, empty spaces are filled
/// using bilinear interpolation.
pub fn rotate_deg(image: &CanvasImage, degree: f64) -> CanvasImage {
    rotate_rad(image, degree / 180f64 * std::f64::consts::PI)
}

#[wasm_bindgen]
pub fn rotate(image: ImageData, degree: f64) -> ImageData {
    let canvas_image = CanvasImage::new(image);

    let rotated = rotate_deg(&canvas_image, degree);
    rotated.into()
}