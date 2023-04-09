use std::ops;
use itertools::iproduct;
use num_traits::Zero;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::ImageData;
use crate::canvas_image::CanvasImage;
use crate::image_index::{reflective_indexed, ZeroPaddedImage};
use crate::utils;

#[wasm_bindgen]
pub struct Kernel {
    data: Vec<f64>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum BorderStrategy {
    Zero,
    Circular,
}

#[wasm_bindgen]
pub fn convolve(image: ImageData, kernel: &Kernel, border_strategy: BorderStrategy) -> ImageData {
    utils::set_panic_hook();

    let image = CanvasImage::new(image);
    let mut buffer = Vec::with_capacity((image.height() * image.width() * 4) as usize);


    match border_strategy {
        BorderStrategy::Zero => {
            for y in 0..image.height() as isize {
                for x in 0..image.width() as isize {
                    let mut r_acc = 0f64;
                    let mut g_acc = 0f64;
                    let mut b_acc = 0f64;
                    let mut a_acc = 0f64;


                    // let r = image.r(x as u32, y as u32).unwrap_or(0);
                    // let g = image.g(x as u32, y as u32).unwrap_or(0);
                    // let b = image.b(x as u32, y as u32).unwrap_or(0);
                    // let a = image.a(x as u32, y as u32).unwrap_or(0);


                    for i in -((kernel.height as isize) / 2)..=(kernel.height / 2) as isize {
                        for j in -((kernel.width as isize) / 2)..=(kernel.width / 2) as isize {
                            let r = ZeroPaddedImage::r(&image, (x - i) as i32, (y - j) as i32);
                            let g = ZeroPaddedImage::g(&image, (x - i) as i32, (y - j) as i32);
                            let b = ZeroPaddedImage::b(&image, (x - i) as i32, (y - j) as i32);
                            let a = ZeroPaddedImage::a(&image, (x - i) as i32, (y - j) as i32);

                            r_acc += kernel[(i, j)] * r as f64;
                            g_acc += kernel[(i, j)] * g as f64;
                            b_acc += kernel[(i, j)] * b as f64;
                            a_acc += kernel[(i, j)] * a as f64;
                        }
                    }

                    buffer.push(r_acc.clamp(0.0, 255.0) as u8);
                    buffer.push(g_acc.clamp(0.0, 255.0) as u8);
                    buffer.push(b_acc.clamp(0.0, 255.0) as u8);
                    buffer.push(a_acc.clamp(0.0, 255.0) as u8);


                    // buffer.push(r);
                    // buffer.push(g);
                    // buffer.push(b);
                    // buffer.push(a);
                }
            }
        }
        BorderStrategy::Circular => todo!(),
    }

    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut buffer), image.width(), image.height()).unwrap()
}


impl CanvasImage {
    pub fn convolve(&self, kernel: &Kernel) -> Vec<f64> {
        let mut buffer = vec![0f64; (self.height() * self.width() * 4) as usize];
        for y in 0..self.height() as isize {
            for x in 0..self.width() as isize {
                let mut r_acc = 0f64;
                let mut g_acc = 0f64;
                let mut b_acc = 0f64;
                let mut a_acc = 0f64;


                for i in -((kernel.height as isize) / 2)..=(kernel.height / 2) as isize {
                    for j in -((kernel.width as isize) / 2)..=(kernel.width / 2) as isize {
                        let r = ZeroPaddedImage::r(self, (x - i) as i32, (y - j) as i32);
                        let g = ZeroPaddedImage::g(self, (x - i) as i32, (y - j) as i32);
                        let b = ZeroPaddedImage::b(self, (x - i) as i32, (y - j) as i32);
                        let a = ZeroPaddedImage::a(self, (x - i) as i32, (y - j) as i32);

                        r_acc += kernel[(i, j)] * r as f64;
                        g_acc += kernel[(i, j)] * g as f64;
                        b_acc += kernel[(i, j)] * b as f64;
                        a_acc += kernel[(i, j)] * a as f64;
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
        let new_height = (self.height() as usize + kernel.height - 1).next_power_of_two();
        let new_width = (self.width() as usize + kernel.width - 1).next_power_of_two();

        // we only care about the r channel
        let r_access = |x, y| self.r(x, y);
        let mut image_complex: Vec<_> = iproduct!(0..new_height, 0..new_width)
            .map(|(x, y)| {
                if (0 <= x && x <= self.width() as usize) &&
                    (0 <= y && y <= self.height() as usize) {
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
        let mut buffer = vec![0f64; (self.height() * self.width() * 4) as usize];
        for y in 0..self.height() as usize {
            for x in 0..self.width() as usize {
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