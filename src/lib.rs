mod utils;
mod fn_extensions;
mod single_pixel_transformation;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
/// An RGBA image. The data is stored in a flat array in row-major order. Exactly like how ImageData
/// is stored in the browser.
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




