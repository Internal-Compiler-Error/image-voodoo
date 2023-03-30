use std::slice::ChunksExactMut;
use super::*;

/// An iterator over the RGBA values of an image. Goes from left to right, top to bottom.
pub struct RBGAIterator<'a> {
    image: &'a CanvasImage,
    /// The current x position, we should read from this before incrementing it.
    x: u32,
    /// The current y position, we should read from this before incrementing it.
    y: u32,
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
    image: &'a CanvasImage,
    /// The current x position, we should read from this before incrementing it.
    x: u32,
    /// The current y position, we should read from this before incrementing it.
    y: u32,
    /// how much we need to add to the base index to get the correct channel
    offset: u8,
}

impl Iterator for ChannelIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = 4 * (self.y * self.image.width + self.x) as usize;
        let value = self.image.data.get(offset + self.offset as usize).cloned();

        self.x += 1;
        if self.x >= self.image.width {
            self.x = 0;
            self.y += 1;
        }
        value
    }
}

impl Iterator for RBGAIterator<'_> {
    type Item = (u8, u8, u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.image.r(self.x, self.y)?;
        let g = self.image.g(self.x, self.y)?;
        let b = self.image.b(self.x, self.y)?;
        let a = self.image.a(self.x, self.y)?;

        self.x += 1;
        if self.x >= self.image.width {
            self.x = 0;
            self.y += 1;
        }

        Some((r, g, b, a))
    }
}


impl CanvasImage {
    /// returns an iterator over the RGBA values of the image
    pub fn rgba_iter(&self) -> RBGAIterator {
        RBGAIterator {
            image: self,
            x: 0,
            y: 0,
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
            image: self,
            x: 0,
            y: 0,
            offset: 0,
        }
    }

    /// returns an iterator over the green channel
    pub fn g_iter(&self) -> ChannelIterator {
        ChannelIterator {
            image: self,
            x: 0,
            y: 0,
            offset: 1,
        }
    }

    /// returns an iterator over the blue channel
    pub fn b_iter(&self) -> ChannelIterator {
        ChannelIterator {
            image: self,
            x: 0,
            y: 0,
            offset: 2,
        }
    }

    /// honestly don't know why you would ever want an iterator over the alpha channel but ok
    pub fn a_iter(&self) -> ChannelIterator {
        ChannelIterator {
            image: self,
            x: 0,
            y: 0,
            offset: 3,
        }
    }
}