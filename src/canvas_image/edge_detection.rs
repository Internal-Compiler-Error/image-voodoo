use crate::canvas_image::CanvasImage;
use enum_iterator::Sequence;
use itertools::iproduct;
use num_traits::{abs_sub};
use crate::convolution::{BorderStrategy, Kernel};
use crate::image_index::reflective_indexed;
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

const WHITE: [u8; 4] = [255, 255, 255, 255];
const BLACK: [u8; 4] = [0, 0, 0, 255];


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Sequence)]
enum Neighbour {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}


/// Iterator that returns pairs of opposing neighbours
struct EightNeighbourIterator {
    x: i32,
    y: i32,
    current: Neighbour,
    done: bool,
}

impl EightNeighbourIterator {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            current: Neighbour::TopLeft,
            done: false,
        }
    }
}


impl Iterator for EightNeighbourIterator {
    type Item = ((i32, i32), (i32, i32));

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }


        // get the next neighbour
        let next = self.current;
        let (cur_x, cur_y) = (self.x, self.y);

        let
            ((x, y), (ops_x, ops_y)) = match next {
            Neighbour::TopLeft =>
                ((cur_x - 1, cur_y - 1), (cur_x + 1, cur_y + 1)),
            Neighbour::Top =>
                ((cur_x - 1, cur_y), (cur_x + 1, cur_y)),
            Neighbour::TopRight =>
                ((cur_x - 1, cur_y + 1), (cur_x + 1, cur_y - 1)),
            Neighbour::Left =>
                ((cur_x, cur_y - 1), (cur_x, cur_y + 1)),
            Neighbour::Right =>
                ((cur_x, cur_y + 1), (cur_x, cur_y - 1)),
            Neighbour::BottomLeft =>
                ((cur_x + 1, cur_y - 1), (cur_x - 1, cur_y + 1)),
            Neighbour::Bottom =>
                ((cur_x + 1, cur_y), (cur_x - 1, cur_y)),
            Neighbour::BottomRight =>
                ((cur_x + 1, cur_y + 1), (cur_x - 1, cur_y - 1)),
        };

        // if we are at the bottom right, we are done
        match self.current.next() {
            Some(next_dir) => {
                self.current = next_dir;
            }
            None => self.done = true,
        }


        Some(((x, y), (ops_x, ops_y)))
    }
}


impl CanvasImage {
    #[allow(dead_code)]
    /// ONLY for greyscale
    /// Assuming the image has already gone through the Laplacian matrix, now we just need to find
    /// all the points where the value is close to 0.
    /// # Arguments
    /// * threshold: the tolerance for how close the value needs to be to 0, calculated via
    ///                  `abs(value) > threshold`
    /// # Returns
    ///  a new image where the edges are white and the rest is black
    fn greyscale_laplacian_edges(image: &Vec<f64>,
                                 width: u32,
                                 height: u32,
                                 threshold: f64) -> CanvasImage {

        // access the r channel of the image, we assume the image is greyscale so all the channels
        // are the same except for the alpha channel
        let r_access = |x, y| {
            let x = x as usize;
            let y = y as usize;
            let width = width as usize;
            let index = y * width + x;

            let intensity = image
                .as_slice()
                .chunks_exact(4)
                .nth(index)
                .unwrap()[0];

            Some(intensity)
        };

        let r_reflect = reflective_indexed::<_, _, i64, _>(&r_access, width, height);

        let rgba = iproduct!(0..height, 0..width)
            .map(|(y, x)| {
                // todo: x and y can go out of bounds of i32
                let neighbours = EightNeighbourIterator::new(x.try_into().unwrap(), y.try_into().unwrap());

                neighbours
                    .map(|((x, y), (ops_x, ops_y))| {
                        let p = r_reflect(x as i64, y as i64);
                        let q = r_reflect(ops_x as i64, ops_y as i64);

                        p.signum() != q.signum() && abs_sub(p, q) > threshold
                    })
                    .reduce(|a, b| a || b)
            })
            .flat_map(|potential_edges|
                if potential_edges.unwrap() {
                    WHITE
                } else {
                    BLACK
                }
            );

        let buffer = Vec::from_iter(rgba);

        CanvasImage::from_vec_with_size(buffer, width, height)
    }


    #[allow(dead_code)]
    /// ONLY for greyscale
    fn greyscale_gradient(
        width: u32,
        height: u32,
        del_x: &Vec<f64>,
        del_y: &Vec<f64>,
        threshold: u32) -> CanvasImage {
        assert_eq!(del_x.len(), del_y.len());
        assert_eq!((width * height) as usize, del_x.len());

        let combined = Iterator::zip(del_x.iter(), del_y.iter());

        let rgba = combined
            // calculate the magnitude of the gradient
            .map(|(x, y)| {
                x.abs() + y.abs()
            })
            .flat_map(|gradient| {
                if gradient > threshold as f64 {
                    WHITE
                } else {
                    BLACK
                }
            });

        let buffer = Vec::from_iter(rgba);

        CanvasImage::from_vec_with_size(buffer, width, height)
    }


    pub fn laplacian_edge(&self, threshold: f64) -> CanvasImage {
        let kernel = Kernel::from_vec(vec![
            1.0, 1.0, 1.0,
            1.0, -8.0, 1.0,
            1.0, 1.0, 1.0,
        ], 3, 3);

        let convolved = self.convolve(&kernel, BorderStrategy::Reflective);

        let edge_map = CanvasImage::greyscale_laplacian_edges(&convolved, self.width, self.height, threshold);
        edge_map
    }

    pub fn laplacian_of_gaussian_edge(&self, threshold: f64) -> CanvasImage {
        let kernel = Kernel::from_vec(vec![
            0.0, 0.0, -01.0, 0.0, 0.0,
            0.0, -1.0, -02.0, -1.0, 0.0,
            -1.0, -2.0, 16.0, -2.0, -1.0,
            0.0, -1.0, -02.0, -1.0, 0.0,
            0.0, 0.0, -01.0, 0.0, 0.0,
        ], 5, 5);

        let convolved = self.convolve(&kernel, BorderStrategy::Reflective);

        let edge_map = CanvasImage::greyscale_laplacian_edges(&convolved, self.width, self.height, threshold);
        edge_map
    }

    // TODO: add noise reduction and edge enhancement
    pub fn sobel_edge(&self, threshold: u32) -> CanvasImage {
        let kernel_x = Kernel::from_vec(vec![
            -1.0, 0.0, 1.0,
            -2.0, 0.0, 2.0,
            -1.0, 0.0, 1.0,
        ], 3, 3);

        let kernel_y = Kernel::from_vec(vec![
            -1.0, -2.0, -1.0,
            0.0, 0.0, 0.0,
            1.0, 2.0, 1.0,
        ], 3, 3);

        let del_x = self.convolve(&kernel_x, BorderStrategy::Reflective);
        let del_y = self.convolve(&kernel_y, BorderStrategy::Reflective);

        CanvasImage::greyscale_gradient(self.width, self.height, &del_x, &del_y, threshold)
    }

    // TODO: add noise reduction and edge enhancement
    pub fn prewitt_edge(&self, threshold: u32) -> CanvasImage {
        let kernel_x = Kernel::from_vec(vec![
            -1.0, 0.0, 1.0,
            -1.0, 0.0, 1.0,
            -1.0, 0.0, 1.0,
        ], 3, 3);

        let kernel_y = Kernel::from_vec(vec![
            -1.0, -1.0, -1.0,
            0.0, 0.0, 0.0,
            1.0, 1.0, 1.0,
        ], 3, 3);

        let del_x = self.convolve(&kernel_x, BorderStrategy::Reflective);
        let del_y = self.convolve(&kernel_y, BorderStrategy::Reflective);

        CanvasImage::greyscale_gradient(self.width, self.height, &del_x, &del_y, threshold)
    }
}

#[wasm_bindgen]
pub fn laplacian_edge(image: ImageData, threshold: f64) -> ImageData {
    let image = CanvasImage::new(image);
    let edge_map = image.laplacian_edge(threshold);
    edge_map.into()
}

#[wasm_bindgen]
pub fn laplacian_of_gaussian_edge(image: ImageData, threshold: f64) -> ImageData {
    let image = CanvasImage::new(image);
    let edge_map = image.laplacian_of_gaussian_edge(threshold);
    edge_map.into()
}


#[cfg(test)]
mod tests {
    use image::{ImageBuffer, Rgba};
    use crate::canvas_image::CanvasImage;
    use crate::canvas_image::edge_detection::EightNeighbourIterator;


    #[test]
    fn laplacian_sanity() {
        // read the picture from file
        let image = image::open("greyscale.png").unwrap();

        // convert to RGBA
        let image = image.into_rgba8();

        // convert to CanvasImage
        let width = image.width();
        let height = image.height();
        let canvas_image = CanvasImage::from_vec_with_size(image.into_raw(), width, height);

        let edge_map = canvas_image.laplacian_edge(300.0);

        // convert to back to image and save
        let image: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(
            edge_map.width(),
            edge_map.height(),
            edge_map.rgba_slice().clone(),
        )
            .unwrap();
        image.save("laplacian.png").unwrap();
    }

    #[test]
    fn laplacian_of_gaussian_sanity() {
        // read the picture from file
        let image = image::open("greyscale.png").unwrap();

        // convert to RGBA
        let image = image.into_rgba8();

        // convert to CanvasImage
        let width = image.width();
        let height = image.height();
        let canvas_image = CanvasImage::from_vec_with_size(image.into_raw(), width, height);

        let edge_map = canvas_image.laplacian_of_gaussian_edge(500.0);

        // convert to back to image and save
        let image: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(
            edge_map.width(),
            edge_map.height(),
            edge_map.rgba_slice().clone(),
        )
            .unwrap();
        image.save("laplacian_of_gaussian.png").unwrap();
    }


    #[test]
    fn sanity() {
        let wtf = EightNeighbourIterator::new(0, 0);
        let wtf = wtf.collect::<Vec<_>>();

        print!("{:?}", wtf)
    }
}