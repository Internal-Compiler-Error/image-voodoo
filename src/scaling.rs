#![allow(unused)]

use crate::canvas_image::CanvasImage;
use crate::interpolation::{lerp, nearest_neighbor};
use itertools::{iproduct, izip};
use std::collections::HashSet;
use std::fmt::format;
use wasm_bindgen::Clamped;
use web_sys::ImageData;
use wasm_bindgen::prelude::*;

pub enum Interpolation {
    Nearest,
    Bilinear,
}

type RGBA = (u8, u8, u8, u8);

fn bi_linear_interpolation(
    top_left: &RGBA,
    top_right: &RGBA,
    bottom_left: &RGBA,
    bottom_right: &RGBA,
    x_diff: f64,
    y_diff: f64,
) -> [u8; 4] {
    // can't iterate over tuples so let's turn them into arrays
    let top_left = [top_left.0, top_left.1, top_left.2, top_left.3];
    let top_right = [top_right.0, top_right.1, top_right.2, top_right.3];
    let bottom_left = [bottom_left.0, bottom_left.1, bottom_left.2, bottom_left.3];
    let bottom_right = [bottom_right.0, bottom_right.1, bottom_right.2, bottom_right.3];

    // another sad fact is we can't guarantee this is a 3 element array
    let mut rgba: Vec<_> = izip!(top_left, top_right, bottom_left, bottom_right)
        .take(3)
        .map(|(top_left, top_right, bottom_left, bottom_right)| {
            let top = lerp(top_left as f64, top_right as f64, x_diff);
            let bottom = lerp(bottom_left as f64, bottom_right as f64, x_diff);

            lerp(top, bottom, y_diff).clamp(0., 255.) as u8
        }).collect();

    // keep the alpha channel as 255;
    rgba.push(255);

    // it's safe because I am not a fucking idiot
    rgba
        .try_into()
        .unwrap()
}

fn bi_nearest_neighbor_interpolation(
    top_left: &RGBA,
    top_right: &RGBA,
    bottom_left: &RGBA,
    bottom_right: &RGBA,
    x_diff: f64,
    y_diff: f64,
) -> [u8; 4] {
    // can't iterate over tuples so let's turn them into arrays
    let top_left = [top_left.0, top_left.1, top_left.2, top_left.3];
    let top_right = [top_right.0, top_right.1, top_right.2, top_right.3];
    let bottom_left = [bottom_left.0, bottom_left.1, bottom_left.2, bottom_left.3];
    let bottom_right = [bottom_right.0, bottom_right.1, bottom_right.2, bottom_right.3];

    // another sad fact is we can't guarantee this is a 3 element array
    let mut rgba: Vec<_> = izip!(top_left, top_right, bottom_left, bottom_right)
        .take(3)
        .map(|(top_left, top_right, bottom_left, bottom_right)| {
            let top = nearest_neighbor(top_left as f64, top_right as f64, x_diff);
            let bottom = nearest_neighbor(bottom_left as f64, bottom_right as f64, x_diff);

            nearest_neighbor(top, bottom, y_diff).clamp(0., 255.) as u8
        }).collect();

    // keep the alpha channel as 255;
    rgba.push(255);

    // it's safe because I am not a fucking idiot
    rgba
        .try_into()
        .unwrap()
}

pub fn scale_bilinear(image: &CanvasImage, new_width: u32, new_height: u32) -> CanvasImage {
    let width = image.width() as f64;
    let height = image.height() as f64;

    // transform the positions into a stream of rbga values we can directly copy into our buffer
    // we don't care if the x y actually have a direct mapping, this implementation of bilinear
    // interpolation will handle that for us
    let rgba = iproduct!(0..new_height, 0..new_width)
        .flat_map(|(y, x)| {
            let x_ratio = x as f64 / (new_width as f64 - 1.);
            let y_ratio = y as f64 / (new_height as f64 - 1.);


            let src_x = (x_ratio * width).min(width);
            let src_y = (y_ratio * height).min(height);


            let x_diff = (src_x - src_x.floor()) / src_x;
            let y_diff = (src_y - src_y.floor()) / src_y;

            let top_left = image.rgba(src_x.floor() as u32, src_y.floor() as u32).unwrap();
            let top_right = image.rgba(src_y.ceil() as u32, src_y.floor() as u32).unwrap();
            let bottom_left = image.rgba(src_x.floor() as u32, src_y.ceil() as u32).unwrap();
            let bottom_right = image.rgba(src_x.ceil() as u32, src_y.ceil() as u32).unwrap();

            bi_linear_interpolation(&top_left, &top_right, &bottom_left, &bottom_right, x_diff, y_diff)
        });

    // then we can just copy our transformed stream into a buffer and create a new image from it
    let buffer = Vec::from_iter(rgba);

    CanvasImage::from_vec_with_size(buffer, new_width, new_height)
}

#[wasm_bindgen]
pub fn scale_via_bilinear(image: ImageData, width_factor: u32, height_factor: u32) -> ImageData {
    let image = CanvasImage::from_image_data(image);

    let new_width = image.horizontal_size() * width_factor;
    let new_height = image.vertical_size() * height_factor;


    let scaled = scale_bilinear(&image, new_width, new_height);
    scaled.into()
}


#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::abs;
    use std::sync::Once;
    use image::{ImageBuffer, Rgba};

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


    #[test]
    fn scale_test() {
        // read the picture from file
        let image = image::open("meme.png").unwrap();

        // convert to RGBA
        let image = image.into_rgba8();

        // convert to CanvasImage
        let width = image.width();
        let height = image.height();
        let mut canvas_image = CanvasImage::from_vec_with_size(image.into_raw(), width, height);

        let scaled = scale_bilinear(&canvas_image, width * 2, height * 2);

        // convert to back to image and save
        let image: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(
            scaled.horizontal_size(),
            scaled.vertical_size(),
            scaled.rgba_slice().clone(),
        )
            .unwrap();
        image.save("meme_scaled.png").unwrap();
    }
}
