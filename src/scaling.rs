#![allow(unused)]

use std::collections::HashSet;
use wasm_bindgen::Clamped;
use web_sys::ImageData;
use crate::canvas_image::CanvasImage;
use crate::interpolation::{bi_nearest_neighbour_interpolation, nearest_neighbour_interpolation};

pub enum Interpolation {
    Nearest,
    Bilinear,
    Bicubic,
}

pub(crate) fn city_block_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    (x1 as f64 - x2 as f64).abs() + (y1 as f64 - y2 as f64).abs()
}

pub(crate) fn chebyshev_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    (x1 as f64 - x2 as f64).abs().max((y1 as f64 - y2 as f64).abs())
}

pub(crate) fn euclidean_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    ((x1 as f64 - x2 as f64).powf(2f64) + (y1 as f64 - y2 as f64).powf(2f64)).sqrt()
}

/// Scales the image to the new width and height, missing pixel values are found using nearest
/// neighbor interpolation
pub(crate) fn nearest(image: &CanvasImage, new_width: u32, new_height: u32) -> CanvasImage {
    let width_scale_factor = new_width / image.width();
    let height_scale_factor = new_height / image.height();

    let mut buffer = vec![0u8; (new_width * new_height * 4) as usize];


    for y in 0..new_height {
        for x in 0..new_width {
            let (r, b, g, a) =

                if x % width_scale_factor == 0 && y % height_scale_factor == 0 {
                    // this means the pixel is in the original image
                    let old_x = x / width_scale_factor;
                    let old_y = y / height_scale_factor;

                    let r = image.r(x, y).unwrap();
                    let g = image.g(x, y).unwrap();
                    let b = image.b(x, y).unwrap();
                    let a = image.a(x, y).unwrap();


                    (r, g, b, a)
                } else {
                    // this means this pixel value has no direct mapping from the original image, and
                    // we need to find the nearest pixel value
                    let x0 = (x as f64 / width_scale_factor as f64).floor() as u32;
                    let y0 = (y as f64 / height_scale_factor as f64).floor() as u32;

                    let x1 = (x as f64 / width_scale_factor as f64).ceil() as u32;
                    let y1 = (y as f64 / height_scale_factor as f64).ceil() as u32;

                    let nearest_r = bi_nearest_neighbour_interpolation(x, y, x0, y0, x1, y1, &|x, y| image.r(x, y).unwrap());
                    let nearest_g = bi_nearest_neighbour_interpolation(x, y, x0, y0, x1, y1, &|x, y| image.g(x, y).unwrap());
                    let nearest_b = bi_nearest_neighbour_interpolation(x, y, x0, y0, x1, y1, &|x, y| image.b(x, y).unwrap());
                    let nearest_a = bi_nearest_neighbour_interpolation(x, y, x0, y0, x1, y1, &|x, y| image.a(x, y).unwrap());

                    (nearest_r, nearest_g, nearest_b, nearest_a)
                };

            let index = (y * new_width + x) as usize * 4;
            buffer[index] = r;
            buffer[index + 1] = g;
            buffer[index + 2] = b;
            buffer[index + 3] = a;
        }
    }

    let image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut buffer), new_width, new_height).unwrap();

    CanvasImage::new(image)
}
