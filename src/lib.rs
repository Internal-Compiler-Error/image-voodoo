use wasm_bindgen::prelude::*;

pub mod canvas_image;
pub mod convolution;
pub mod histogram;
pub mod interpolation;
pub mod affine;
pub mod scaling;
pub mod single_pixel_transformation;
pub mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
