#![allow(dead_code)]

use crate::canvas_image::CanvasImage;
use itertools::{iproduct, Itertools, MinMaxResult};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::ImageData;
use nalgebra::{Matrix2, Matrix3, Vector2, Vector3};

const TRANSPARENT: [u8; 4] = [0, 0, 0, 0];

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

/*********** Matrix Zone ***********/

fn get_operation_matrix(width: usize, height: usize, transformation: &Matrix2<f64>) -> Matrix3<f64> {
    let width = width as f64;
    let height = height as f64;

    let (new_width, new_height) = new_size_after_transformation(width, height, transformation);

    // offset from the center before rotation
    let (cx, cy) = (
        width as f64 / 2f64,
        height as f64 / 2f64);

    // offset from the center after rotation
    let (ox, oy) = (
        (new_width as f64 - width) as f64 / 2.0,
        (new_height as f64 - height) as f64 / 2.0,
    );

    let to_after = Matrix3::new(
        1.0, 0.0, cx,
        0.0, 1.0, cy,
        0.0, 0.0, 1.0,
    );


    let to_before = Matrix3::new(
        1.0, 0.0, -cx - ox,
        0.0, 1.0, -cy - oy,
        0.0, 0.0, 1.0,
    );

    // convert the transformation matrix to 3x3 matrix
    let transformation = Matrix3::new(
        transformation[(0, 0)], transformation[(0, 1)], 0.0,
        transformation[(1, 0)], transformation[(1, 1)], 0.0,
        0.0, 0.0, 1.0,
    );

    to_after * transformation * to_before
}


fn rotate_via_matrix(image: &CanvasImage, radian: f64) -> CanvasImage {
    let width = image.width() as f64;
    let height = image.height() as f64;

    let (new_width, new_height) = width_height_after_rotation_matrix(radian, width, height);

    let rotate = Matrix2::new(
        radian.cos(), -radian.sin(),
        radian.sin(), radian.cos(),
    );
    let transform = get_operation_matrix(image.width() as usize,
                                         image.height() as usize,
                                         &rotate);


    let rgba = iproduct!(0..new_height, 0..new_width)
        // map to vector
        .map(|(y, x)| Vector3::new(x as f64, y as f64, 1.0))
        // do everything in one step
        .map(|v| transform * v)
        // map to intensity
        .flat_map(|v| {
            let x = v.x.round() as i32;
            let y = v.y.round() as i32;

            if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
                image.rgba(x as u32, y as u32)
                    .map_or(TRANSPARENT, |(r, g, b, a)| [r, g, b, a])
            } else {
                TRANSPARENT
            }
        });

    let buffer = Vec::from_iter(rgba);
    CanvasImage::from_vec_with_size(buffer, new_width, new_height)
}

/// Shears the image according to
/// [1 + lambda * miu, lambda,
///  miu             , 1]
fn shear(image: &CanvasImage, lambda: f64, miu: f64) -> CanvasImage {
    let width = image.width() as f64;
    let height = image.height() as f64;

    let shear = Matrix2::new(
        1.0 + lambda * miu, lambda,
        miu, 1.0,
    );

    let (new_width, new_height) = new_size_after_transformation(width, height, &shear);

    let transform = get_operation_matrix(image.width() as usize,
                                         image.height() as usize,
                                         &shear);

    let rgba = iproduct!(0..new_height, 0..new_width)
        // map to vector
        .map(|(y, x)| Vector3::new(x as f64, y as f64, 1.0))
        // do everything in one step
        .map(|v| transform * v)
        // map to intensity
        .flat_map(|v| {
            let x = v.x.round() as i32;
            let y = v.y.round() as i32;

            if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
                image.rgba(x as u32, y as u32)
                    .map_or(TRANSPARENT, |(r, g, b, a)| [r, g, b, a])
            } else {
                TRANSPARENT
            }
        });

    let buffer = Vec::from_iter(rgba);
    CanvasImage::from_vec_with_size(buffer, new_width, new_height)
}

fn new_size_after_transformation(width: f64, height: f64, transformation: &Matrix2<f64>) -> (u32, u32) {
    let corners = [
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, height),
        Vector2::new(width, 0.0),
        Vector2::new(width, height)];
    let transformed_corners = corners
        .iter()
        .map(|v| transformation * v)
        .map(|v| (v.x, v.y));

    let x_minmax = transformed_corners
        .clone()
        .map(|(x, _)| x).minmax();
    let y_minmax = transformed_corners
        .clone()
        .map(|(_, y)| y).minmax();

    let (x_min, x_max) = min_max(&x_minmax);
    let (y_min, y_max) = min_max(&y_minmax);

    let new_width = (x_max - x_min).ceil() as u32 + 1;
    let new_height = (y_max - y_min).ceil() as u32 + 1;

    (new_width, new_height)
}

fn width_height_after_rotation_matrix(radian: f64, width: f64, height: f64) -> (u32, u32) {
    let rotation_matrix = Matrix2::new(
        radian.cos(), -radian.sin(),
        radian.sin(), radian.cos(),
    );

    new_size_after_transformation(width, height, &rotation_matrix)
}

#[wasm_bindgen]
pub fn rotate(image: ImageData, degree: f64) -> ImageData {
    let canvas_image = CanvasImage::new(image);

    let rotated = rotate_deg(&canvas_image, degree);
    rotated.into()
}

#[wasm_bindgen]
pub fn shear_wasm(image: ImageData, lambda: f64, miu: f64) -> ImageData {
    let canvas_image = CanvasImage::new(image);

    let sheared = shear(&canvas_image, lambda, miu);
    sheared.into()
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

    #[test]
    fn sanity_matrix() {
        // read the picture from file
        let image = image::open("meme.png").unwrap();

        // convert to RGBA
        let image = image.into_rgba8();

        // convert to CanvasImage
        let width = image.width();
        let height = image.height();
        let canvas_image = CanvasImage::from_vec_with_size(image.into_raw(), width, height);

        let degrees = 45.0;
        let radian = degrees * std::f64::consts::PI / 180.0;
        let rotated = rotate_via_matrix(&canvas_image, radian);

        // convert to back to image and save
        let image: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(
            rotated.width(),
            rotated.height(),
            rotated.rgba_slice().clone(),
        )
            .unwrap();
        image.save("meme_rotated_matrix.png").unwrap();
    }

    #[test]
    fn sanity_shear() {
        // read the picture from file
        let image = image::open("meme.png").unwrap();

        // convert to RGBA
        let image = image.into_rgba8();

        // convert to CanvasImage
        let width = image.width();
        let height = image.height();
        let canvas_image = CanvasImage::from_vec_with_size(image.into_raw(), width, height);


        let sheared = shear(&canvas_image, 1.0, 1.0);

        // convert to back to image and save
        let image: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(
            sheared.width(),
            sheared.height(),
            sheared.rgba_slice().clone(),
        )
            .unwrap();
        image.save("meme_sheared.png").unwrap();
    }
}
