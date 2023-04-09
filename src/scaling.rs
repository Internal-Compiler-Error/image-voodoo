#![allow(unused)]

use crate::canvas_image::CanvasImage;
use crate::interpolation::{
    bi_nearest_neighbour_interpolation, bilinear_interpolation, nearest_neighbour_interpolation,
};
use itertools::iproduct;
use std::collections::HashSet;
use std::fmt::format;
use wasm_bindgen::Clamped;
use web_sys::ImageData;
use wasm_bindgen::prelude::*;

pub enum Interpolation {
    Nearest,
    Bilinear,
}

pub(crate) fn city_block_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    (x1 as f64 - x2 as f64).abs() + (y1 as f64 - y2 as f64).abs()
}

pub(crate) fn chebyshev_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    (x1 as f64 - x2 as f64)
        .abs()
        .max((y1 as f64 - y2 as f64).abs())
}

pub(crate) fn euclidean_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> f64 {
    ((x1 as f64 - x2 as f64).powf(2f64) + (y1 as f64 - y2 as f64).powf(2f64)).sqrt()
}


pub fn scale_bilinear(image: &CanvasImage, new_width: u32, new_height: u32) -> CanvasImage {
    let width_scale_factor = (new_width - 1) / (image.width() - 1);
    let height_scale_factor = (new_height - 1) / (image.height() - 1);

    let pos = iproduct!(0..new_height, 0..new_width);

    // transform the positions into a stream of rbga values we can directly copy into our buffer
    // we don't care if the x y actually have a direct mapping, this implementation of bilinear
    // interpolation will handle that for us
    let rgba = pos.flat_map(|(y, x)| {
        let xf = x as f64;
        let yf = y as f64;
        let width_scale_factor_f = width_scale_factor as f64;
        let height_scale_factor_f = height_scale_factor as f64;

        let (x0, x1) = {
            let x0 = (xf / width_scale_factor_f).floor() as u32;
            let x1 = (xf / width_scale_factor_f).ceil() as u32;
            (x0, x1)
        };

        let (y0, y1) = {
            let y0 = (yf / height_scale_factor_f).floor() as u32;
            let y1 = (yf / height_scale_factor_f).ceil() as u32;
            (y0, y1)
        };

        let xx = xf / width_scale_factor_f;
        let yy = yf / height_scale_factor_f;
        let nearest_r = bilinear_interpolation(xx, yy, x0, y0, x1, y1, &|x, y| {
            image
                .r(x, y)
                .expect(&*format!("{x} {y} should be within bounds"))
        });
        let nearest_g = bilinear_interpolation(xx, yy, x0, y0, x1, y1, &|x, y| {
            image
                .g(x, y)
                .expect(&*format!("{x} {y} should be within bounds"))
        });
        let nearest_b = bilinear_interpolation(xx, yy, x0, y0, x1, y1, &|x, y| {
            image
                .b(x, y)
                .expect(&*format!("{x} {y} should be within bounds"))
        });
        let nearest_a = bilinear_interpolation(xx, yy, x0, y0, x1, y1, &|x, y| {
            image
                .a(x, y)
                .expect(&*format!("{x} {y} should be within bounds"))
        });

        [nearest_r, nearest_g, nearest_b, nearest_a]
    });

    // then we can just copy our transformed stream into a buffer and create a new image from it
    let buffer = Vec::from_iter(rgba);

    CanvasImage::from_vec_with_size(buffer, new_width, new_height)
}

#[wasm_bindgen]
pub fn scale_via_bilinear(image: ImageData, width_factor: u32, height_factor: u32) -> ImageData {
    let image = CanvasImage::new(image);

    let new_width = image.width() * width_factor;
    let new_height = image.height() * height_factor;


    let scaled = scale_bilinear(&image, new_width, new_height);
    scaled.into()
}


#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::abs;
    use std::sync::Once;

    static INIT: Once = Once::new();

    #[test]
    fn bilinear_scale() -> color_eyre::Result<()> {
        INIT.call_once(|| color_eyre::install().unwrap());

        // data stolen from:
        // https://theailearner.com/2018/12/29/image-processing-bilinear-interpolation/

        #[rustfmt::skip]
            let image: Vec<u8> = vec![
            10, 10, 10, 0, 20, 20, 20, 0,
            30, 30, 30, 0, 40, 40, 40, 0,
        ];

        let image = CanvasImage::from_vec_with_size(image, 2, 2);
        let scaled = scale_bilinear(&image, 4, 4);

        #[rustfmt::skip]
            let expected: Vec<u8> = vec![
            10, 10, 10, 0,
            12, 12, 12, 0,
            17, 17, 17, 0,
            20, 20, 20, 0,
            15, 15, 15, 0,
            17, 17, 17, 0,
            22, 22, 22, 0,
            25, 25, 25, 0,
            25, 25, 25, 0,
            27, 27, 27, 0,
            32, 32, 32, 0,
            35, 35, 35, 0,
            30, 30, 30, 0,
            32, 32, 32, 0,
            37, 37, 37, 0,
            40, 40, 40, 0,
        ];

        // as long as the difference is less than 5 we're good
        for (a, b) in expected.iter().zip(scaled.rgba_slice().iter()) {
            assert!(abs(*a as i32 - *b as i32) < 5);
        }

        Ok(())
    }
}
