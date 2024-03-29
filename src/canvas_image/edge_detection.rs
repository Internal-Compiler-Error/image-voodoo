use crate::canvas_image::CanvasImage;
use crate::convolution::{BorderStrategy, Kernel};
use crate::image_index::reflective_indexed;
use enum_iterator::Sequence;
use itertools::{iproduct, izip};
use num_traits::abs_sub;
use rand::distributions::{Bernoulli, Distribution};
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

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

        let ((x, y), (ops_x, ops_y)) = match next {
            Neighbour::TopLeft => ((cur_x - 1, cur_y - 1), (cur_x + 1, cur_y + 1)),
            Neighbour::Top => ((cur_x - 1, cur_y), (cur_x + 1, cur_y)),
            Neighbour::TopRight => ((cur_x - 1, cur_y + 1), (cur_x + 1, cur_y - 1)),
            Neighbour::Left => ((cur_x, cur_y - 1), (cur_x, cur_y + 1)),
            Neighbour::Right => ((cur_x, cur_y + 1), (cur_x, cur_y - 1)),
            Neighbour::BottomLeft => ((cur_x + 1, cur_y - 1), (cur_x - 1, cur_y + 1)),
            Neighbour::Bottom => ((cur_x + 1, cur_y), (cur_x - 1, cur_y)),
            Neighbour::BottomRight => ((cur_x + 1, cur_y + 1), (cur_x - 1, cur_y - 1)),
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
    /// ONLY for greyscale
    /// Assuming the image has already gone through the Laplacian matrix, now we just need to find
    /// all the points where the value is close to 0.
    /// # Arguments
    /// * threshold: the tolerance for how close the value needs to be to 0, calculated via
    ///                  `abs(value) > threshold`
    /// # Returns
    ///  a new image where the edges are white and the rest is black
    fn greyscale_laplacian_edges(
        convolved_image: &Vec<f64>,
        width: u32,
        height: u32,
        threshold: f64,
    ) -> CanvasImage {
        // access the entire rgba pixel of the convolution image
        let pixel_access = |x, y| {
            let x = x as usize;
            let y = y as usize;
            let width = width as usize;
            let index = y * width + x;

            let rgba = convolved_image
                .as_slice()
                .chunks_exact(4)
                .nth(index)
                .unwrap();

            Some([rgba[0], rgba[1], rgba[2], rgba[3]])
        };

        let pixel_reflect = reflective_indexed::<_, _, i64, _>(&pixel_access, width, height);

        let rgba = iproduct!(0..height, 0..width)
            .map(|(y, x)| {
                let neighbours =
                    EightNeighbourIterator::new(x.try_into().unwrap(), y.try_into().unwrap());

                neighbours
                    .map(|((x, y), (ops_x, ops_y))| {
                        let [r, g, b, a] = pixel_reflect(x as i64, y as i64);
                        let [r_ops, g_ops, b_ops, a_ops] =
                            pixel_reflect(ops_x as i64, ops_y as i64);

                        // test to see if each channel is an edge
                        [
                            r.signum() != r_ops.signum() && abs_sub(r, r_ops) > threshold,
                            g.signum() != g_ops.signum() && abs_sub(g, g_ops) > threshold,
                            b.signum() != b_ops.signum() && abs_sub(b, b_ops) > threshold,
                            a.signum() != a_ops.signum() && abs_sub(a, a_ops) > threshold,
                        ]
                    })
                    // we consider a pixel is an edge as long as one or more direction crosses zero
                    .reduce(|[r1, g1, b1, a1], [r2, g2, b2, a2]| {
                        [r1 || r2, g1 || g2, b1 || b2, a1 || a2]
                    })
                    .unwrap()
            })
            .flat_map(|[r, g, b, _a]| {
                [
                    if r { 255 } else { 0 },
                    if g { 255 } else { 0 },
                    if b { 255 } else { 0 },
                    255,
                ]
            });

        let buffer = Vec::from_iter(rgba);

        CanvasImage::from_vec_with_size(buffer, width, height)
    }

    /// Assuming the image has already gone through a type of gradient kernel, with x direction in `del_x` and y
    /// direction in `del_y`, now just mark the edge by if the magnitude of the gradient is greater than the
    /// `threshold`.
    fn gradient_edge_localization(
        width: u32,
        height: u32,
        del_x: &Vec<f64>,
        del_y: &Vec<f64>,
        threshold: u32,
    ) -> CanvasImage {
        assert_eq!(del_x.len(), del_y.len());
        // 4 channels in the del_x and del_y, one for each channel, so we need to multiply by 4
        assert_eq!((width * height) as usize * 4, del_x.len());

        let combined = Iterator::zip(
            del_x.as_slice().chunks_exact(4),
            del_y.as_slice().chunks_exact(4),
        );

        let rgba = combined
            // calculate the magnitude of the gradient
            .map(|(x, y)| {
                // you can't collect into an array because the size is not known at compile time
                let magnitudes: Vec<_> = izip!(x.iter(), y.iter())
                    .map(|(x, y)| x.abs() + y.abs())
                    .collect();

                [magnitudes[0], magnitudes[1], magnitudes[2], magnitudes[3]]
            })
            .flat_map(|[r, g, b, _a]| {
                [
                    if r > threshold as f64 { 255 } else { 0 },
                    if g > threshold as f64 { 255 } else { 0 },
                    if b > threshold as f64 { 255 } else { 0 },
                    255, // just assume the alpha channel is always 255
                ]
            });

        let buffer = Vec::from_iter(rgba);

        CanvasImage::from_vec_with_size(buffer, width, height)
    }

    pub fn laplacian_edge(&self, threshold: f64) -> CanvasImage {
        let kernel = Kernel::from_vec(vec![1.0, 1.0, 1.0, 1.0, -8.0, 1.0, 1.0, 1.0, 1.0], 3, 3);

        let convolved = self.convolve(&kernel, BorderStrategy::Reflective);

        let edge_map =
            CanvasImage::greyscale_laplacian_edges(&convolved, self.width, self.height, threshold);
        edge_map
    }

    pub fn laplacian_of_gaussian_edge(&self, threshold: f64) -> CanvasImage {
        let kernel = Kernel::from_vec(
            vec![
                0.0, 0.0, -01.0, 0.0, 0.0, 0.0, -1.0, -02.0, -1.0, 0.0, -1.0, -2.0, 16.0, -2.0,
                -1.0, 0.0, -1.0, -02.0, -1.0, 0.0, 0.0, 0.0, -01.0, 0.0, 0.0,
            ],
            5,
            5,
        );

        let convolved = self.convolve(&kernel, BorderStrategy::Reflective);

        let edge_map =
            CanvasImage::greyscale_laplacian_edges(&convolved, self.width, self.height, threshold);
        edge_map
    }

    // TODO: add noise reduction and edge enhancement
    pub fn sobel_edge(&self, threshold: u32) -> CanvasImage {
        let kernel_x = Kernel::from_vec(vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0], 3, 3);

        let kernel_y = Kernel::from_vec(vec![-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0], 3, 3);

        let del_x = self.convolve(&kernel_x, BorderStrategy::Reflective);
        let del_y = self.convolve(&kernel_y, BorderStrategy::Reflective);

        CanvasImage::gradient_edge_localization(self.width, self.height, &del_x, &del_y, threshold)
    }

    // TODO: add noise reduction and edge enhancement
    pub fn prewitt_edge(&self, threshold: u32) -> CanvasImage {
        let kernel_x = Kernel::from_vec(vec![-1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0], 3, 3);

        let kernel_y = Kernel::from_vec(vec![-1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 3, 3);

        let del_x = self.convolve(&kernel_x, BorderStrategy::Reflective);
        let del_y = self.convolve(&kernel_y, BorderStrategy::Reflective);

        CanvasImage::gradient_edge_localization(self.width, self.height, &del_x, &del_y, threshold)
    }

    /// Set noise to the image by performing a bernoulli trial for each pixel with probability p. If the trial succeeds,
    /// `noise` is *set* as the pixel for *all* color channels.
    pub fn set_bernoulli_noise(&mut self, p: f64, noise: u8) -> color_eyre::Result<()> {
        assert!(p >= 0.0 && p <= 1.0, "p must be in [0, 1]");
        let mut rng = rand::thread_rng();
        let bernoulli = Bernoulli::new(p)?;

        // for each pixel, perform a bernoulli trial with probability p, if true, set pixel to noise
        self.rgba_iter_mut()
            .filter(|_| bernoulli.sample(&mut rng))
            .for_each(|(r, g, b, _a)| {
                *r = noise;
                *g = noise;
                *b = noise;
            });

        Ok(())
    }

    pub fn add_salt(&mut self, p: f64) -> color_eyre::Result<()> {
        self.set_bernoulli_noise(p, 255)
    }

    pub fn add_pepper(&mut self, p: f64) -> color_eyre::Result<()> {
        self.set_bernoulli_noise(p, 0)
    }
}

#[wasm_bindgen]
pub fn laplacian_edge(image: ImageData, threshold: f64) -> ImageData {
    let image = CanvasImage::from_image_data(image);
    let edge_map = image.laplacian_edge(threshold);
    edge_map.into()
}

#[wasm_bindgen]
pub fn laplacian_of_gaussian_edge(image: ImageData, threshold: f64) -> ImageData {
    let image = CanvasImage::from_image_data(image);
    let edge_map = image.laplacian_of_gaussian_edge(threshold);
    edge_map.into()
}

#[wasm_bindgen]
pub fn add_salt(image: ImageData, p: f64) -> Result<ImageData, String> {
    let mut image = CanvasImage::from_image_data(image);
    image.add_salt(p).map_err(|e| e.to_string())?;
    Ok(image.into())
}

#[wasm_bindgen]
pub fn add_pepper(image: ImageData, p: f64) -> Result<ImageData, String> {
    let mut image = CanvasImage::from_image_data(image);
    image.add_pepper(p).map_err(|e| e.to_string())?;
    Ok(image.into())
}

#[wasm_bindgen]
pub fn prewitt_edge(image: ImageData, threshold: f64) -> ImageData {
    let image = CanvasImage::from_image_data(image);
    let edge_map = image.prewitt_edge(threshold as u32);
    edge_map.into()
}

#[wasm_bindgen]
pub fn sobel_edge(image: ImageData, threshold: f64) -> ImageData {
    let image = CanvasImage::from_image_data(image);
    let edge_map = image.sobel_edge(threshold as u32);
    edge_map.into()
}

#[cfg(test)]
mod tests {
    use crate::canvas_image::edge_detection::EightNeighbourIterator;
    use crate::canvas_image::CanvasImage;
    use image::{ImageBuffer, Rgba};

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
            edge_map.horizontal_size(),
            edge_map.vertical_size(),
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
            edge_map.horizontal_size(),
            edge_map.vertical_size(),
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
