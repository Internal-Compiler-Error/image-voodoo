use std::sync::Once;
use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;

pub mod canvas_image;
pub mod convolution;
pub mod histogram;
pub mod interpolation;
pub mod affine;
pub mod scaling;
pub mod single_pixel_transformation;
pub mod utils;
mod image_index;

const INIT: Once = Once::new();

#[wasm_bindgen]
pub fn init() {
    INIT.call_once(|| {
        set_panic_hook();
    });
}



// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
