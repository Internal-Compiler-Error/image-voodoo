use std::ops;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
struct Kernel {
    data: Vec<f32>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl Kernel {
    fn from_vec(data: Vec<f32>, width: usize, height: usize) -> Kernel {
        Kernel { data, width, height }
    }
}

impl ops::Index<(i32, i32)> for Kernel {
    type Output = f32;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        let half_width = self.width as i32 / 2;
        let half_height = self.height as i32 / 2;

        let x = x + half_width;
        let y = y + half_height;
        assert!(x >= 0 && x < self.width as i32);
        assert!(y >= 0 && y < self.height as i32);

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