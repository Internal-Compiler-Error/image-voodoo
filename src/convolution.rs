use std::ops;
use itertools::iproduct;
use num_traits::Zero;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::ImageData;
use crate::canvas_image::CanvasImage;
use crate::image_index::{CircularIndexedImage, ReflectiveIndexedImage, ZeroPaddedImage};
use crate::utils;

#[wasm_bindgen]
pub struct Kernel {
    data: Vec<f64>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BorderStrategy {
    Zero,
    Circular,
    Reflective,
}


#[wasm_bindgen]
pub fn convolve(image: ImageData, kernel: &Kernel, border_strategy: BorderStrategy) -> ImageData {
    utils::set_panic_hook();

    let image = CanvasImage::new(image);

    let convolved = image.convolve(kernel, border_strategy);

    // clamp and convert to u8
    let rgba = convolved
        .as_slice()
        .chunks_exact(4)
        .flat_map(|chunk| {
            let r = chunk[0].clamp(0.0, 255.0) as u8;
            let g = chunk[1].clamp(0.0, 255.0) as u8;
            let b = chunk[2].clamp(0.0, 255.0) as u8;
            let a = chunk[3].clamp(0.0, 255.0) as u8;

            [r, g, b, a]
        });

    let mut buffer = Vec::from_iter(rgba);
    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut buffer), image.horizontal_size(), image.vertical_size()).unwrap()
}


impl CanvasImage {
    /// Convolve the image with a kernel, using the specified border strategy.
    pub fn convolve(&self, kernel: &Kernel, border_strategy: BorderStrategy) -> Vec<f64> {
        // some rust lawyer please tell me how to do this better
        let (r, g, b, a): (&dyn Fn(&CanvasImage, i32, i32) -> u8,
                           &dyn Fn(&CanvasImage, i32, i32) -> u8,
                           &dyn Fn(&CanvasImage, i32, i32) -> u8,
                           &dyn Fn(&CanvasImage, i32, i32) -> u8) = match border_strategy {
            BorderStrategy::Zero => (
                &ZeroPaddedImage::r,
                &ZeroPaddedImage::g,
                &ZeroPaddedImage::b,
                &ZeroPaddedImage::a,
            ),
            BorderStrategy::Reflective => (
                &ReflectiveIndexedImage::r,
                &ReflectiveIndexedImage::g,
                &ReflectiveIndexedImage::b,
                &ReflectiveIndexedImage::a,
            ),
            BorderStrategy::Circular => (
                &CircularIndexedImage::r,
                &CircularIndexedImage::g,
                &CircularIndexedImage::b,
                &CircularIndexedImage::a,
            ),
        };


        let mut buffer = Vec::with_capacity((self.vertical_size() * self.horizontal_size() * 4) as usize);
        for y in 0..self.vertical_size() as isize {
            for x in 0..self.horizontal_size() as isize {
                let mut r_acc = 0f64;
                let mut g_acc = 0f64;
                let mut b_acc = 0f64;
                let mut a_acc = 0f64;


                for i in -((kernel.height as isize) / 2)..=(kernel.height / 2) as isize {
                    for j in -((kernel.width as isize) / 2)..=(kernel.width / 2) as isize {
                        let r_intensity = r(&self, (x - i) as i32, (y - j) as i32);
                        let g_intensity = g(&self, (x - i) as i32, (y - j) as i32);
                        let b_intensity = b(&self, (x - i) as i32, (y - j) as i32);
                        let a_intensity = a(&self, (x - i) as i32, (y - j) as i32);

                        r_acc += kernel[(i, j)] * r_intensity as f64;
                        g_acc += kernel[(i, j)] * g_intensity as f64;
                        b_acc += kernel[(i, j)] * b_intensity as f64;

                        // i don't know where in the world would you actually want to mess with
                        // the alpha channel, in a convolution
                        a_acc += a_intensity as f64;
                    }
                }

                buffer.push(r_acc);
                buffer.push(g_acc);
                buffer.push(b_acc);
                buffer.push(a_acc);
            }
        }

        buffer
    }


    /// ONLY for greyscale and it zero pads, would usually doesn't make the output look too good
    fn greyscale_convolve_with_fft(&self, kernel: &Kernel) -> Vec<f64> {
        let new_height = (self.vertical_size() as usize + kernel.height - 1).next_power_of_two();
        let new_width = (self.horizontal_size() as usize + kernel.width - 1).next_power_of_two();

        // we only care about the r channel
        let r_access = |x, y| self.r(x, y);
        let mut image_complex: Vec<_> = iproduct!(0..new_height, 0..new_width)
            .map(|(x, y)| {
                if (0 <= x && x <= self.horizontal_size() as usize) &&
                    (0 <= y && y <= self.vertical_size() as usize) {
                    let real = r_access(x as u32, y as u32).unwrap() as f64;

                    Complex::new(real, 0.0)
                } else {
                    Complex::zero()
                }
            })
            .collect();


        let kernel_access = |x, y| kernel.data[y as usize + x as usize * kernel.width];
        let mut kernel_complex: Vec<_> = iproduct!(0..new_height, 0..new_width)
            .map(|(x, y)| {
                if (0 <= x && x <= kernel.width as usize) &&
                    (0 <= y && y <= kernel.height as usize) {
                    let real = kernel_access(x as u32, y as u32);

                    Complex::new(real, 0.0)
                } else {
                    Complex::zero()
                }
            })
            .collect();


        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(new_height * new_width);

        fft.process(&mut image_complex);
        fft.process(&mut kernel_complex);

        // multiply the two
        for (image, kernel) in image_complex.iter_mut().zip(kernel_complex.iter()) {
            *image *= kernel;
        }

        // inverse fft
        let ifft = planner.plan_fft_inverse(new_height * new_width);
        ifft.process(&mut image_complex);

        // convert to greyscale
        let mut buffer = vec![0f64; (self.vertical_size() * self.horizontal_size() * 4) as usize];
        for y in 0..self.vertical_size() as usize {
            for x in 0..self.horizontal_size() as usize {
                let r = image_complex[x * new_width + y].re;
                let g = r;
                let b = r;
                let a = 0f64;

                buffer.push(r);
                buffer.push(g);
                buffer.push(b);
                buffer.push(a);
            }
        }

        buffer
    }
}

#[wasm_bindgen]
impl Kernel {
    pub fn from_vec(data: Vec<f64>, width: usize, height: usize) -> Kernel {
        Kernel { data, width, height }
    }
}

/// Convert from the kernel view of index that goes from the center to the edges to the actual
/// index in the data array.
impl ops::Index<(isize, isize)> for Kernel {
    type Output = f64;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        let half_width = (self.width / 2) as isize;
        let half_height = (self.height / 2) as isize;

        let x = x + half_width;
        let y = y + half_height;
        assert!(x >= 0 && x < self.width as isize);
        assert!(y >= 0 && y < self.height as isize);

        let x = x as usize;
        let y = y as usize;

        &self.data[x * self.width + y]
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kernel() {
        let kernel = Kernel::from_vec(vec![
            9., 8., 7.,
            6., 5., 4.,
            3., 2., 1.,
        ], 3, 3);
        assert_eq!(kernel[(0, 0)], 5.);
        assert_eq!(kernel[(-1, -1)], 9.);
        assert_eq!(kernel[(0, 1)], 4.);
        assert_eq!(kernel[(1, 1)], 1.);
        assert_eq!(kernel[(1, 0)], 2.);
    }
}