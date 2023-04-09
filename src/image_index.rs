use num::Integer;
use num_traits::{abs, Signed};
use std::fmt::Debug;
use num::integer::div_floor;

pub trait ZeroPaddedImage {
    fn r(&self, x: i32, y: i32) -> u8;
    fn g(&self, x: i32, y: i32) -> u8;
    fn b(&self, x: i32, y: i32) -> u8;
    fn a(&self, x: i32, y: i32) -> u8;
}

pub trait CircularIndexedImage {
    fn r(&self, x: i32, y: i32) -> u8;
    fn g(&self, x: i32, y: i32) -> u8;
    fn b(&self, x: i32, y: i32) -> u8;
    fn a(&self, x: i32, y: i32) -> u8;
}

pub trait ReflectiveIndexedImage {
    fn r(&self, x: i32, y: i32) -> u8;
    fn g(&self, x: i32, y: i32) -> u8;
    fn b(&self, x: i32, y: i32) -> u8;
    fn a(&self, x: i32, y: i32) -> u8;
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
pub fn circular_indexed<'a, F, U, S>(f: &'a F, x_period: U, y_period: U) -> impl Fn(S, S) -> u8 + 'a
// I am really only interested in the primitive types, so the Copy bound is fine
    where F: Fn(U, U) -> Option<u8>,
          U: Integer + Copy + TryFrom<S> + 'static,
          S: Integer + Copy + Signed,
          <U as TryFrom<S>>::Error: Debug
{
    let unsigned_abs = |x: S| {
        let value = U::try_from(abs(x)).unwrap();
        value
    };

    move |x, y| {
        // can't believe copilot actually suggested this, but I guess it works
        let zero = U::zero();

        let x_offset: U = unsigned_abs(x) % x_period;

        let y_offset: U = unsigned_abs(y) % y_period;


        let x_extra_period: U = if x.is_negative() { x_period } else { zero };
        let y_extra_period: U = if y.is_negative() { y_period } else { zero };


        let x = if x.is_negative() {
            x_period + x_extra_period - x_offset
        } else {
            x_period + x_extra_period + x_offset
        };
        let y = if y.is_negative() {
            y_period + y_extra_period - y_offset
        } else {
            y_period + y_extra_period + y_offset
        };


        f(x % x_period, y % y_period).unwrap()
    }
}


/// Given a image that is only defined on a finite domain, this function will return a function that
/// returns the value of the image at the given point. If the point is outside the domain, the
/// function will return the value reflected across the boundary of the domain.
pub fn reflective_indexed<'a, F, U, S, R>(f: &'a F, x_period: U, y_period: U) -> impl Fn(S, S) -> R + 'a
    where F: Fn(U, U) -> Option<R>,
          U: Integer + Copy + TryFrom<S> + 'static,
          S: Integer + Copy + Signed + 'static,
          <U as TryFrom<S>>::Error: Debug {
    let unsigned_abs = |x: S| {
        U::try_from(abs(x)).unwrap()
    };

    let one = U::one();

    move |x, y| {
        let zero = U::zero();
        let x_floored_period = div_floor(unsigned_abs(x), x_period);
        let y_floored_period = div_floor(unsigned_abs(y), y_period);

        let x_offset = unsigned_abs(x) % x_period;
        let y_offset = unsigned_abs(y) % y_period;


        let x_base: U = if x_floored_period.is_even() {
            zero
        } else {
            x_period
        };

        let y_base = if y_floored_period.is_even() {
            zero
        } else {
            y_period
        };

        let x = if x_floored_period.is_even() {
            x_base + x_offset
        } else {
            x_base - x_offset
        };

        let y = if y_floored_period.is_even() {
            y_base + y_offset
        } else {
            y_base - y_offset
        };

        f(x % x_period, y % y_period).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflective_indexing() {
        // a function that is only defined on [0, 4) x [0, 4)
        let f = |x: u32, y: u32| {
            if x < 4 && y < 4 {
                Some(x + y)
            } else {
                None
            }
        };

        let g = reflective_indexed::<_, _, i32, _>(&f, 4, 4);

        for i in -4..4 {
            for j in -4..4 {
                println!("g({}, {}) = {}", i, j, g(i, j));
            }
        }
    }
}