use float_cmp::approx_eq;
use crate::fn_extensions::{ChannelIterator};

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

    /// Normalize the histogram so that the sum of all buckets is 1
    pub fn normalize(&mut self) {
        let bucket_count = self
            .buckets.iter()
            .filter(|x| **x != 0 as f64).count();

        self
            .buckets
            .iter_mut()
            .for_each(|x| *x /= bucket_count as f64);

        #[cfg(debug_assertions)]
        {
            let sum: f64 = self.buckets.iter().sum();
            debug_assert!(approx_eq!(f64, sum, 1.0));
        }
    }
}

