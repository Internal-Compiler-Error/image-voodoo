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
use crate::image_index::{CircularIndexedImage, ZeroPaddedImage};

/// Consumes the canvas image and returns an ImageData for the browser
impl Into<ImageData> for CanvasImage {
    fn into(mut self) -> ImageData {
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut self.data), self.width, self.height).unwrap()
    }
}

impl CanvasImage {
    /// Returns a new image that is flipped horizontally. In other words, the image is mirrored along
    /// the vertical axis.
    pub fn flip_horizontal(&self) -> ImageData {
        let mut data = Vec::new();

        for y in 0..self.height {
            for x in (0..self.width).rev() {
                data.push(self.r(x, y).unwrap());
                data.push(self.g(x, y).unwrap());
                data.push(self.b(x, y).unwrap());
                data.push(self.a(x, y).unwrap());
            }
        }

        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), self.width, self.height).unwrap()
    }

    /// Returns a new image that is flipped vertically. In other words, the image is mirrored along
    /// the horizontal axis.
    pub fn flip_vertical(&self) -> ImageData {
        let mut data = Vec::new();

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                data.push(self.r(x, y).unwrap());
                data.push(self.g(x, y).unwrap());
                data.push(self.b(x, y).unwrap());
                data.push(self.a(x, y).unwrap());
            }
        }

        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), self.width, self.height).unwrap()
    }


    /**************************** random junk **************************************/

    pub fn new(image_data: ImageData) -> CanvasImage {
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
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

    pub fn equalize(&self) -> ImageData {
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
                let r_freq = r_bucket[chunk[0] as usize];
                let g_freq = g_bucket[chunk[1] as usize];
                let b_freq = b_bucket[chunk[2] as usize];

                chunk[0] = (r_freq * 255.0).clamp(0.0, 255.0) as u8;
                chunk[1] = (g_freq * 255.0).clamp(0.0, 255.0) as u8;
                chunk[2] = (b_freq * 255.0).clamp(0.0, 255.0) as u8;
            });

        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(image.as_slice()),
            self.width,
            self.height,
        ).unwrap()
    }
}

impl CircularIndexedImage for CanvasImage {
    fn r(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }


        let width = self.width();
        let height = self.height();

        let f = |x, y| self.r(x, y);


        let value = image_index::circular_indexed(&f, width, height)(x, y);
        value
    }

    fn g(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }


        let f = |x, y| self.g(x, y);
        let f = image_index::circular_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn b(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }


        let f = |x, y| self.b(x, y);
        let f = image_index::circular_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn a(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

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

mod edge_detection;
pub use edge_detection::*;

#[cfg(test)]
mod test {
    use crate::image_index::zero_padded;
    use super::*;


    #[test]
    fn zero_padded_returns_zero() {
        // only defined from 0 to 6 for both x and y
        let f = |x, y| {
            if 0 <= x && x < 7 && 0 <= y && y < 7 {
                Some(x as u8 * y as u8)
            } else {
                None
            }
        };

        let g = zero_padded(&f);

        assert_eq!(g(0, 0), 0);
        assert_eq!(g(3, 4), 12);
        assert_eq!(g(100, 100), 0);
        assert_eq!(g(-1, -1), 0);
    }

    //
    // #[test]
    // fn test_reflective_indexed() {
    //     // only defined from -3 to 3 for both x and y
    //     let f = |x: i32, y: i32| {
    //         if -3 <= x && x < 4 && -3 <= y && y < 4 {
    //             Some((x.abs() * y.abs()) as u8)
    //         } else {
    //             None
    //         }
    //     };
    //
    //     // yes the period is indeed 7, since 0 is part of -3 to 3
    //     let g = reflective_indexed(&f, 7, 7);
    //
    //     assert_eq!(g(0, 0), 0);
    //     assert_eq!(g(3, 3), 9);
    //
    //     assert_eq!(g(3, 4), 3 * 3);
    //     assert_eq!(g(-5, -4), 2 * 3);
    // }
}