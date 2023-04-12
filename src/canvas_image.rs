use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::ImageData;
use crate::histogram::Histogram;


pub struct CanvasImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

mod iterator;

pub use iterator::*;
use crate::image_index;
use crate::image_index::{CircularIndexedImage, ReflectiveIndexedImage, ZeroPaddedImage};

/// Consumes the canvas image and returns an ImageData for the browser
impl Into<ImageData> for CanvasImage {
    fn into(mut self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut self.data), self.width, self.height).unwrap()
    }
}

impl From<ImageData> for CanvasImage {
    fn from(image_data: ImageData) -> Self {
        CanvasImage::from_image_data(image_data)
    }
}

impl CanvasImage {
    /**************************** random junk **************************************/

    pub fn from_image_data(image_data: ImageData) -> CanvasImage {
        CanvasImage {
            data: image_data.data().0,
            width: image_data.width(),
            height: image_data.height(),
        }
    }

    pub fn from_vec_with_size(data: Vec<u8>, width: u32, height: u32) -> CanvasImage {
        CanvasImage {
            data,
            width,
            height,
        }
    }

    pub fn rgba_slice(&self) -> &[u8] {
        &self.data
    }

    /// Returns the *geometric* width of the image
    ///
    /// With a pixel of 1 x 1, the geometric width is 0. Since points in the geometric space are
    /// have no sizes.
    pub fn width(&self) -> u32 {
        self.width - 1
    }

    /// Returns the *geometric* height of the image
    ///
    /// With a pixel of 1 x 1, the geometric height is 0. Since points in the geometric space are
    /// have no sizes.
    pub fn height(&self) -> u32 {
        self.height - 1
    }

    /// Returns the number of pixels in the horizontal direction
    pub fn horizontal_size(&self) -> u32 {
        self.width
    }

    /// Returns the number of pixels in the vertical direction
    pub fn vertical_size(&self) -> u32 {
        self.height
    }

    /**************************** single pixel accessors ****************************/

    pub fn r(&self, x: u32, y: u32) -> Option<u8> {
        let offset = 4 * (y * self.width + x) as usize;
        self.data.get(offset).cloned()
    }

    pub fn g(&self, x: u32, y: u32) -> Option<u8> {
        let offset = 4 * (y * self.width + x) as usize;
        self.data.get(offset + 1).cloned()
    }

    pub fn b(&self, x: u32, y: u32) -> Option<u8> {
        let offset = 4 * (y * self.width + x) as usize;
        self.data.get(offset + 2).cloned()
    }

    pub fn a(&self, x: u32, y: u32) -> Option<u8> {
        let offset = 4 * (y * self.width + x) as usize;
        self.data.get(offset + 3).cloned()
    }

    pub fn rgba(&self, x: u32, y: u32) -> Option<(u8, u8, u8, u8)> {
        let rgba = (self.r(x, y), self.g(x, y), self.b(x, y), self.a(x, y));

        match rgba {
            (Some(r), Some(g), Some(b), Some(a)) => Some((r, g, b, a)),
            _ => None,
        }
    }

    /***************************** histograms *********************************/
    pub fn blue_histogram(&self) -> Histogram {
        let mut b_channel = self.b_iter();
        Histogram::from_channel_iterator(&mut b_channel)
    }

    pub fn green_histogram(&self) -> Histogram {
        let mut g_channel = self.g_iter();
        Histogram::from_channel_iterator(&mut g_channel)
    }

    pub fn red_histogram(&self) -> Histogram {
        let mut r_channel = self.r_iter();
        Histogram::from_channel_iterator(&mut r_channel)
    }


    /// I am 99% sure this is useless in most cases
    pub fn alpha_histogram(&self) -> Histogram {
        let mut a_channel = self.a_iter();
        Histogram::from_channel_iterator(&mut a_channel)
    }

    /// Equalize the distribution of intensities in the image, each channel except for alpha is treated independently
    pub fn equalize(&self) -> CanvasImage {
        let mut image = self.data.clone();

        let mut r_channel = self.r_iter();
        let mut g_channel = self.g_iter();
        let mut b_channel = self.b_iter();

        let r_hist = Histogram::from_channel_iterator(&mut r_channel)
            .cumulative_normalized();
        let g_hist = Histogram::from_channel_iterator(&mut g_channel)
            .cumulative_normalized();
        let b_hist = Histogram::from_channel_iterator(&mut b_channel)
            .cumulative_normalized();

        let r_bucket = r_hist.bucket();
        let g_bucket = g_hist.bucket();
        let b_bucket = b_hist.bucket();

        image
            .as_mut_slice()
            .chunks_exact_mut(4)
            .for_each(|chunk| {
                let r_freq = r_bucket[chunk[0] as usize] * 255.0;
                let g_freq = g_bucket[chunk[1] as usize] * 255.0;
                let b_freq = b_bucket[chunk[2] as usize] * 255.0;

                chunk[0] = (r_freq).clamp(0.0, 255.0) as u8;
                chunk[1] = (g_freq).clamp(0.0, 255.0) as u8;
                chunk[2] = (b_freq).clamp(0.0, 255.0) as u8;


                // println!("{} {} {}", chunk[0], chunk[1], chunk[2]);
            });

        CanvasImage::from_vec_with_size(image, self.width, self.height)
    }

    /// convert an color image to a greyscale image using the luminance method from
    /// sRGB -> Linear RGB -> Luminance -> sRGB
    /// alpha is left untouched
    pub fn convert_to_greyscale(&mut self) {
        self
            .rgba_iter_mut()
            .for_each(|(r, g, b, _)| {
                let mut linear_r = *r as f64;
                let mut linear_g = *g as f64;
                let mut linear_b = *b as f64;

                linear_r = linear_r.linearize();
                linear_g = linear_g.linearize();
                linear_b = linear_b.linearize();

                let mut lumma = to_luminance(linear_r, linear_g, linear_b);
                lumma = to_srgb(lumma);

                *r = lumma.clamp(u8::MIN as f64, u8::MAX as f64) as u8;
                *g = lumma.clamp(u8::MIN as f64, u8::MAX as f64) as u8;
                *b = lumma.clamp(u8::MIN as f64, u8::MAX as f64) as u8;
            });
    }
}

impl CircularIndexedImage for CanvasImage {
    fn r(&self, x: i32, y: i32) -> u8 {
        let f = |x, y| self.r(x, y);
        let f = image_index::circular_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn g(&self, x: i32, y: i32) -> u8 {
        let f = |x, y| self.g(x, y);
        let f = image_index::circular_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn b(&self, x: i32, y: i32) -> u8 {
        let f = |x, y| self.b(x, y);
        let f = image_index::circular_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn a(&self, x: i32, y: i32) -> u8 {
        let f = |x, y| self.a(x, y);
        let f = image_index::circular_indexed(&f, self.width, self.height);
        f(x, y)
    }
}

impl ZeroPaddedImage for CanvasImage {
    fn r(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.r(x, y);
        let f = image_index::zero_padded(&f);
        f(x, y)
    }

    fn g(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.g(x, y);
        let f = image_index::zero_padded(&f);
        f(x, y)
    }

    fn b(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.b(x, y);
        let f = image_index::zero_padded(&f);
        f(x, y)
    }

    fn a(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.a(x, y);
        let f = image_index::zero_padded(&f);
        f(x, y)
    }
}

impl ReflectiveIndexedImage for CanvasImage {
    fn r(&self, x: i32, y: i32) -> u8 {
        let f = |x, y| self.r(x, y);
        let f = image_index::reflective_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn g(&self, x: i32, y: i32) -> u8 {
        let f = |x, y| self.g(x, y);
        let f = image_index::reflective_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn b(&self, x: i32, y: i32) -> u8 {
        let f = |x, y| self.b(x, y);
        let f = image_index::reflective_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn a(&self, x: i32, y: i32) -> u8 {
        let f = |x, y| self.a(x, y);
        let f = image_index::reflective_indexed(&f, self.width, self.height);
        f(x, y)
    }
}

mod edge_detection;
mod filters;
mod crop;

pub use filters::*;
pub use edge_detection::*;
use crate::color_space::{Linearize, to_luminance, to_srgb};


#[wasm_bindgen]
pub fn greyscale(image: ImageData) -> ImageData {
    let mut canvas_image = CanvasImage::from_image_data(image);
    canvas_image.convert_to_greyscale();
    canvas_image.into()
}