use super::*;
use std::slice::{ChunksExact, ChunksExactMut};

/// An iterator over the RGBA values of an image. Goes from left to right, top to bottom.
pub struct RBGAIterator<'a> {
    /// The iterator over the chunks of the image data.
    iter: ChunksExact<'a, u8>,
}

pub struct RBGAIteratorMut<'a> {
    chunk_iter: ChunksExactMut<'a, u8>,
}

impl<'a> Iterator for RBGAIteratorMut<'a> {
    type Item = (&'a mut u8, &'a mut u8, &'a mut u8, &'a mut u8);

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.chunk_iter.next()?;

        let (r, s) = chunk.split_first_mut()?;
        let (g, s) = s.split_first_mut()?;
        let (b, s) = s.split_first_mut()?;
        let (a, _) = s.split_first_mut()?;

        Some((r, g, b, a))
    }
}

/// An iterator over a single channel of an image. Goes from left to right, top to bottom.
pub struct ChannelIterator<'a> {
    iter: ChunksExact<'a, u8>,
    /// how much we need to add to the base index to get the correct channel
    offset: u8,
}

impl Iterator for ChannelIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|chunk| chunk[self.offset as usize])
    }
}

impl Iterator for RBGAIterator<'_> {
    type Item = (u8, u8, u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|chunk| {
            let r = chunk[0];
            let g = chunk[1];
            let b = chunk[2];
            let a = chunk[3];

            (r, g, b, a)
        })
    }
}

impl CanvasImage {
    /// returns an iterator over the RGBA values of the image
    pub fn rgba_iter(&self) -> RBGAIterator {
        RBGAIterator {
            iter: self.data.chunks_exact(4),
        }
    }

    pub fn rgba_iter_mut(&mut self) -> RBGAIteratorMut {
        RBGAIteratorMut {
            chunk_iter: self.data.chunks_exact_mut(4),
        }
    }

    /// returns an iterator over the red channel
    pub fn r_iter(&self) -> ChannelIterator {
        ChannelIterator {
            iter: self.data.chunks_exact(4),
            offset: 0,
        }
    }

    /// returns an iterator over the green channel
    pub fn g_iter(&self) -> ChannelIterator {
        ChannelIterator {
            iter: self.data.chunks_exact(4),
            offset: 1,
        }
    }

    /// returns an iterator over the blue channel
    pub fn b_iter(&self) -> ChannelIterator {
        ChannelIterator {
            iter: self.data.chunks_exact(4),
            offset: 2,
        }
    }

    /// honestly don't know why you would ever want an iterator over the alpha channel but ok
    pub fn a_iter(&self) -> ChannelIterator {
        ChannelIterator {
            iter: self.data.chunks_exact(4),
            offset: 3,
        }
    }
}
