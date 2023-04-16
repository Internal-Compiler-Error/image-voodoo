use crate::utils::set_panic_hook;
use std::sync::Once;
use wasm_bindgen::prelude::*;

pub mod affine;
pub mod canvas_image;
pub mod color_space;
pub mod convolution;
pub mod distance;
pub mod flip;
pub mod histogram;
pub mod image_index;
pub mod interpolation;
pub mod scaling;
pub mod single_pixel_transformation;
pub mod utils;

const INIT: Once = Once::new();

#[wasm_bindgen]
pub fn init() {
    INIT.call_once(|| {
        set_panic_hook();
    });
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
