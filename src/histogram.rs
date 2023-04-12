use float_cmp::approx_eq;
use web_sys::ImageData;
use crate::canvas_image::{CanvasImage, ChannelIterator};

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Histogram {
    // it's f64 because we need to normalize it
    buckets: Vec<f64>,
}

impl Histogram {
    pub(crate) fn from_channel_iterator(channel: &mut ChannelIterator) -> Self {
        let mut buckets = vec![0f64; 256];

        for intensity in channel {
            buckets[intensity as usize] += 1f64;
        }

        Histogram { buckets }
    }

    /// Returns the number of pixels in the image that have the given intensity
    pub fn bucket(&self) -> &[f64] {
        &self.buckets
    }

    /// Normalize the histogram so that the sum of all buckets is 1,
    /// this assumes that the histogram *is* cumulative
    pub fn normalize(&mut self) {
        let sum = self.buckets.iter().sum::<f64>();

        self
            .buckets
            .iter_mut()
            .for_each(|x| *x /= sum);

        #[cfg(debug_assertions)]
        debug_assert!(self.is_normalized());
    }

    /// Build a *new* histogram that is the cumulative histogram of the current histogram
    pub fn cumulative(&self) -> Histogram {
        // cumulative and scans are the same thing
        let cumulative: Vec<_> =
            self.buckets.iter().scan(0f64, |sum, e| {
                *sum += e;
                Some(*sum)
            }).collect();

        Histogram { buckets: cumulative }
    }


    /// Build a *new* histogram that is the cumulative histogram of the current histogram and then
    /// normalize it
    pub fn cumulative_normalized(&self) -> Histogram {
        let mut hist = self.clone();
        hist.normalize();
        hist.cumulative()
    }

    pub fn is_normalized(&self) -> bool {
        let sum: f64 = self.buckets.iter().sum();
        approx_eq!(f64, sum, 1.0, epsilon = 0.0001)
    }

    /// Unlike the normalized, we can't really check if the histogram is cumulative, so we just
    /// check if the last bucket is 1
    pub fn is_cumulative(&self) -> bool {
        let last = self.buckets.last().unwrap();
        approx_eq!(f64, *last, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use image::{ImageBuffer, Rgba};
    use crate::canvas_image::CanvasImage;

    #[test]
    fn sanity() {
        // read the picture from file
        let image = image::open("car.png").unwrap();

        // convert to RGBA
        let image = image.into_rgba8();

        // convert to CanvasImage
        let width = image.width();
        let height = image.height();
        let canvas_image = CanvasImage::from_vec_with_size(image.into_raw(), width, height);

        let equalized = canvas_image.equalize();

        // convert to back to image and save
        let image: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(
            equalized.horizontal_size(),
            equalized.vertical_size(),
            equalized.rgba_slice().clone(),
        )
            .unwrap();
        image.save("car_eq.png").unwrap();
    }
}


#[wasm_bindgen]
pub fn equalize(image: ImageData) -> ImageData {
    let image = CanvasImage::from_image_data(image);

    let equalized = image.equalize();

    equalized.into()
}