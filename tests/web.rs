//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;
use web_sys::ImageData;
use wasm_bindgen::Clamped;
use image_voodoo::fn_extensions::CanvasImage;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn r_access_work_as_expected() {
    // not this is in RGBA format
    let image: Vec<u8> = vec![
        0, 1, 2, 0,
        3, 4, 5, 0,
        6, 7, 8, 0,
    ];


    let image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&image), 1, 3).unwrap();
    let image = CanvasImage::new(&image);

    assert_eq!(image.r(0, 0), Some(0));
    assert_eq!(image.r(1, 0), Some(3));
    assert_eq!(image.r(2, 0), Some(6));
    assert_eq!(image.r(3, 3), None);
}

#[wasm_bindgen_test]
fn g_access_work_as_expected() {
    // not this is in RGBA format
    let image: Vec<u8> = vec![
        0, 1, 2, 0,
        3, 4, 5, 0,
        6, 7, 8, 0,
    ];


    let image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&image), 1, 3).unwrap();
    let image = CanvasImage::new(&image);

    assert_eq!(image.g(0, 0), Some(1));
    assert_eq!(image.g(1, 0), Some(4));
    assert_eq!(image.g(2, 0), Some(7));
    assert_eq!(image.g(3, 3), None);
}

#[wasm_bindgen_test]
fn b_access_work_as_expected() {
    // not this is in RGBA format
    let image: Vec<u8> = vec![
        0, 1, 2, 0,
        3, 4, 5, 0,
        6, 7, 8, 0,
    ];


    let image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&image), 1, 3).unwrap();
    let image = CanvasImage::new(&image);

    assert_eq!(image.b(0, 0), Some(2));
    assert_eq!(image.b(1, 0), Some(5));
    assert_eq!(image.b(2, 0), Some(8));
    assert_eq!(image.b(3, 3), None);
}
