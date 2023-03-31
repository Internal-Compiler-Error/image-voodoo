use crate::canvas_image::CanvasImage;
use itertools::iproduct;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::ImageData;

/// Rotate the image by radian, enlarge the image to fit the rotated image, empty spaces are filled
/// using bilinear interpolation.
pub fn rotate_rad(image: &CanvasImage, radian: f64) -> CanvasImage {
    let width = image.width() as f64;
    let height = image.height() as f64;

    let (new_width, new_height) = {
        // calculate the rotated image bounds
        let corners = [(0.0, 0.0), (0.0, height), (width, 0.0), (width, height)];
        let rotated_corners = corners.iter().map(|&(x, y)| {
            let new_x = x * radian.cos() - y * radian.sin();
            let new_y = x * radian.sin() + y * radian.cos();
            (new_x, new_y)
        });
        let min_x = rotated_corners
            .clone()
            .map(|(x, _)| x)
            .fold(f64::INFINITY, f64::min);
        let max_x = rotated_corners
            .clone()
            .map(|(x, _)| x)
            .fold(f64::NEG_INFINITY, f64::max);
        let min_y = rotated_corners
            .clone()
            .map(|(_, y)| y)
            .fold(f64::INFINITY, f64::min);
        let max_y = rotated_corners
            .clone()
            .map(|(_, y)| y)
            .fold(f64::NEG_INFINITY, f64::max);

        // calculate the size of the new image
        let new_width = (max_x - min_x).ceil() as u32 + 1;
        let new_height = (max_y - min_y).ceil() as u32 + 1;

        (new_width, new_height)
    };

    // image center before rotation
    let (cx, cy) = (image.width() as f64 / 2f64, image.height() as f64 / 2f64);

    // offset from the center after rotation
    let (ox, oy) = (
        (new_width as f64 - width) / 2.0,
        (new_height as f64 - height) / 2.0,
    );

    let rgba = iproduct!(0..new_height, 0..new_width).flat_map(|(y, x)| {
        let x = x as f64 - cx - ox;
        let y = cy - y as f64 + oy;

        // rotate the pixel coordinates
        let new_x = x * radian.cos() - y * radian.sin();
        let new_y = x * radian.sin() + y * radian.cos();

        let x = (new_x + cx).round() as i32;
        let y = (cy - new_y).round() as i32;

        let pixel = if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
            image.rgba(x as u32, y as u32).unwrap_or((255, 0, 0, 0))
        } else {
            (255, 0, 0, 0)
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    #[test]
    fn sanity() {
        // read the picture from file
        let image = image::open("meme.png").unwrap();

        // convert to RGBA
        let image = image.into_rgba8();

        // convert to CanvasImage
        let width = image.width();
        let height = image.height();
        let canvas_image = CanvasImage::from_vec_with_size(image.into_raw(), width, height);

        let rotated = rotate_deg(&canvas_image, 45.0);

        // convert to back to image and save
        let image: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(
            rotated.width(),
            rotated.height(),
            rotated.rgba_slice().clone(),
        )
        .unwrap();
        image.save("meme_rotated.png").unwrap();
    }
}
