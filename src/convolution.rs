use std::ops;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::ImageData;
use crate::canvas_image::CanvasImage;
use crate::image_index::ZeroPaddedImage;
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
                for x in 0..image.width()  as isize{
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