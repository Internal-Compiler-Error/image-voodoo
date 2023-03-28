use std::marker::PhantomData;
use num_traits::int::PrimInt;
use wasm_bindgen::Clamped;
use web_sys::ImageData;
use crate::histogram::Histogram;

pub struct CanvasImage<'a> {
    data: Clamped<Vec<u8>>,
    width: u32,
    height: u32,
    _marker: PhantomData<&'a ImageData>,
}

/// An iterator over the RGBA values of an image. Goes from left to right, top to bottom.
pub struct RBGAIterator<'a> {
    image: &'a CanvasImage<'a>,
    /// The current x position, we should read from this before incrementing it.
    x: u32,
    /// The current y position, we should read from this before incrementing it.
    y: u32,
}

/// An iterator over a single channel of an image. Goes from left to right, top to bottom.
pub struct ChannelIterator<'a> {
    image: &'a CanvasImage<'a>,
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

impl CanvasImage<'_> {
    /**************************** random junk **************************************/

    pub fn new(image_data: &ImageData) -> CanvasImage {
        CanvasImage {
            data: image_data.data(),
            width: image_data.width(),
            height: image_data.height(),
            _marker: PhantomData,
        }
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

    /**************************** intensity iterators ****************************/

    /// returns an iterator over the RGBA values of the image
    pub fn rgba_iter(&self) -> RBGAIterator {
        RBGAIterator {
            image: self,
            x: 0,
            y: 0,
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
}

pub trait ZeroPaddedImage {
    fn r(&self, x: i32, y: i32) -> u8;
    fn g(&self, x: i32, y: i32) -> u8;
    fn b(&self, x: i32, y: i32) -> u8;
    fn a(&self, x: i32, y: i32) -> u8;
}

trait CircularIndexedImage {
    fn r(&self, x: i32, y: i32) -> u8;
    fn g(&self, x: i32, y: i32) -> u8;
    fn b(&self, x: i32, y: i32) -> u8;
    fn a(&self, x: i32, y: i32) -> u8;
}

impl CircularIndexedImage for CanvasImage<'_> {
    fn r(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.r(x, y);
        let f = circular_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn g(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.g(x, y);
        let f = circular_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn b(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.b(x, y);
        let f = circular_indexed(&f, self.width, self.height);
        f(x, y)
    }

    fn a(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.a(x, y);
        let f = circular_indexed(&f, self.width, self.height);
        f(x, y)
    }
}

impl ZeroPaddedImage for CanvasImage<'_> {
    fn r(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.r(x, y);
        let f = zero_padded(&f);
        f(x, y)
    }

    fn g(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.g(x, y);
        let f = zero_padded(&f);
        f(x, y)
    }

    fn b(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.b(x, y);
        let f = zero_padded(&f);
        f(x, y)
    }

    fn a(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x = x as u32;
        let y = y as u32;

        let f = |x, y| self.a(x, y);
        let f = zero_padded(&f);
        f(x, y)
    }
}

/// Given a image that is only defined on a finite domain, this function will return a function that
/// returns the value of the image at the given point. If the point is outside the domain, the
/// function will return 0
///
/// F: the function that returns the value of the image at a given point
/// C: the type of the coordinates of the image, such as u32, i32, etc.
pub fn zero_padded<'a, F, C>(f: &'a F) -> impl Fn(C, C) -> u8 + 'a
    where F: Fn(C, C) -> Option<u8>,
{
    move |x, y| {
        match f(x, y) {
            Some(v) => v,
            None => 0,
        }
    }
}

/// Given a image that is only defined on a finite domain, this function will return a function that
/// returns the value of the image at the given point. If the point is outside the domain, the
/// function will return the value of the image at the corresponding point in the domain.
pub fn circular_indexed<'a, F, C>(f: &'a F, x_period: C, y_period: C) -> impl Fn(C, C) -> u8 + 'a
    where F: Fn(C, C) -> Option<u8>,
          C: PrimInt + 'static
{
    move |x, y| {
        let x = x % x_period;
        let y = y % y_period;

        // by taking the modulus, we will be inside the domain of definition
        f(x, y).unwrap()
    }
}


// /// Given a image that is only defined on a finite domain, this function will return a function that
// /// returns the value of the image at the given point. If the point is outside the domain, the
// /// function will return the value reflected across the boundary of the domain.
// pub fn reflective_indexed<'a, F>(f: &'a F, x_period: i32, y_period: i32) -> impl Fn(i32, i32) -> u8 + 'a
//     where F: Fn(i32, i32) -> Option<u8>,
// {
//     // move |x: i32, y: i32| {
//     //     let x = x_period - (x % x_period).abs();
//     //     let y = y_period - (y % y_period).abs();
//     //
//     //     // by taking the modulus, we will be inside the domain of definition
//     //     f(x, y).unwrap()
//     // };
//     todo!()
// }

#[cfg(test)]
mod test {
    use super::*;
    use ImageData;

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