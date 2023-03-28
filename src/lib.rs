pub mod utils;
pub mod fn_extensions;
pub mod single_pixel_transformation;
pub mod histogram;
pub mod convolution;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
/// An RGBA image. The data is stored in a flat array in row-major order. Exactly like how ImageData
/// is stored in the browser. You almost never need to use this struct directly. Instead, use the
/// `CanvasImage` struct. Which is a wrapper around this struct. This only exists because
/// wasm-bindgen doesn't do lifetimes
pub struct ConcreteImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}




