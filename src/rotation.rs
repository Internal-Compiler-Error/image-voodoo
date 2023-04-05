use crate::canvas_image::CanvasImage;
use itertools::{iproduct, Itertools, MinMaxResult};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::ImageData;

/// Find the minimum and maximum value of an iterator. If the iterator is empty, it will panic.
fn min_max<T>(result: &MinMaxResult<T>) -> (T, T)
    where
        T: Copy
{
    match result {
        MinMaxResult::OneElement(min_max) => {
            (*min_max, *min_max)
        }
        MinMaxResult::MinMax(min, max) => {
            (*min, *max)
        }
        _ => unreachable!()
    }
}

/// Rotate a point by radian
fn rotate_point(point: (f64, f64), radian: f64) -> (f64, f64) {
    let (x, y) = point;
    let new_x = x * radian.cos() + y * radian.sin();
    let new_y = -x * radian.sin() + y * radian.cos();
    (new_x, new_y)
}

/// Rotate the image by radian, enlarge the image to fit the rotated image, empty spaces are filled
/// using bilinear interpolation.
pub fn rotate_rad(image: &CanvasImage, radian: f64) -> CanvasImage {
    let width = image.width() as f64;
    let height = image.height() as f64;

    let (new_width, new_height) = width_height_after_rotation(radian, width, height);

    // image center before rotation
    let (cx, cy) = (image.width() as f64 / 2f64, image.height() as f64 / 2f64);

    // offset from the center after rotation
    let (ox, oy) = (
        (new_width as f64 - width) / 2.0,
        (new_height as f64 - height) / 2.0,
    );

    let rgba = iproduct!(0..new_height, 0..new_width).flat_map(|(y, x)| {
        // pixel coordinates before rotation
        let x = x as f64 - cx - ox;
        let y = y as f64 - cy - oy;

        // rotate the pixel coordinates
        let (new_x, new_y) = rotate_point((x, y), radian);

        // pixel coordinates after rotation
        let x = (new_x + cx).round() as i32;
        let y = (new_y + cy).round() as i32;

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

fn width_height_after_rotation(radian: f64, width: f64, height: f64) -> (u32, u32) {
    let (new_width, new_height) = {
        // calculate the rotated image bounds
        let corners = [(0.0, 0.0), (0.0, height), (width, 0.0), (width, height)];
        let rotated_corners = corners.iter().map(|&(x, y)| rotate_point((x, y), radian));

        let x_minmax = rotated_corners
            .clone()
            .map(|(x, _)| x).minmax();
        let y_minmax = rotated_corners
            .clone()
            .map(|(_, y)| y).minmax();

        let (x_min, x_max) = min_max(&x_minmax);
        let (y_min, y_max) = min_max(&y_minmax);

        // calculate the size of the new image
        let new_width = (x_max - x_min).ceil() as u32 + 1;
        let new_height = (y_max - y_min).ceil() as u32 + 1;

        (new_width, new_height)
    };
    (new_width, new_height)
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
