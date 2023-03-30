use wasm_bindgen::Clamped;
use web_sys::ImageData;
use crate::fn_extensions::CanvasImage;

pub enum Interpretation {
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

    for x in 0..image.width() {
        for y in 0..image.height() {
            let r = image.r(x, y).unwrap();
            let g = image.g(x, y).unwrap();
            let b = image.b(x, y).unwrap();
            let a = image.a(x, y).unwrap();

            for x_offset in 0..width_scale_factor {
                for y_offset in 0..height_scale_factor {
                    let new_x = x * width_scale_factor + x_offset;
                    let new_y = y * height_scale_factor + y_offset;

                    let index = (new_x + new_y * new_width) * 4;

                    buffer[index as usize] = r;
                    buffer[index as usize + 1] = g;
                    buffer[index as usize + 2] = b;
                    buffer[index as usize + 3] = a;
                }
            }
        }
    }

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut buffer), new_width, new_height).unwrap();
    CanvasImage::new(&image_data)
}