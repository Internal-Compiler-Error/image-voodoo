use float_cmp::approx_eq;
use crate::canvas_image::{ChannelIterator};

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
        let mut hist = self.cumulative();
        hist.normalize();
        hist
    }

    pub fn is_normalized(&self) -> bool {
        let sum: f64 = self.buckets.iter().sum();
        approx_eq!(f64, sum, 1.0)
    }

    /// Unlike the normalized, we can't really check if the histogram is cumulative, so we just
    /// check if the last bucket is 1
    pub fn is_cumulative(&self) -> bool {
        let last = self.buckets.last().unwrap();
        approx_eq!(f64, *last, 1.0)
    }
}

