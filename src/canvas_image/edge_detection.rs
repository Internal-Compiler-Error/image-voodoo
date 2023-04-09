use crate::canvas_image::CanvasImage;
use enum_iterator::Sequence;
use itertools::iproduct;
use num_traits::{abs, abs_sub};
use crate::image_index::reflective_indexed;

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
    x: u32,
    y: u32,
    current: Neighbour,
    done: bool,
}

impl EightNeighbourIterator {
    fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            current: Neighbour::TopLeft,
            done: false,
        }
    }
}


impl Iterator for EightNeighbourIterator {
    type Item = ((u32, u32), (u32, u32));

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
    /// * tolerance: the tolerance for how close the value needs to be to 0, calculated via
    ///                  `abs(value) < tolerance`
    /// # Returns
    ///  a new image where the edges are white and the rest is black
    pub fn greyscale_laplacian_edges(&self, tolerance: u32) -> CanvasImage {
        let r_access = |x, y| self.r(x, y);
        let r_reflect = reflective_indexed::<_, _, i64>(&r_access, self.width, self.height);

        let rgba = iproduct!(0..self.height, 0..self.width)
            .map(|(x, y)| {
                let neighbours = EightNeighbourIterator::new(x, y);

                neighbours
                    .map(|((x, y), (ops_x, ops_y))| abs_sub(r_reflect(x as i64, y as i64) as i64, r_reflect(ops_x as i64, ops_y as i64) as i64) < tolerance as i64)
                    .reduce(|a, b| a || b)
            })
            .flat_map(|potential_edges|
                if potential_edges.unwrap() {
                    [255, 255, 255, 0]
                } else {
                    [0, 0, 0, 0]
                }
            );

        let buffer = Vec::from_iter(rgba);

        CanvasImage::from_vec_with_size(buffer, self.width, self.height)
    }


    /// ONLY for greyscale
    pub fn greyscale_prewitt(del_x: &CanvasImage, del_y: &CanvasImage, tolerance: u32) -> CanvasImage {
        assert!(del_x.width == del_y.width && del_x.height == del_y.height);
        let combined = Iterator::zip(del_x.rgba_iter(), del_y.rgba_iter());

        // combined.map(|(x, y)| {
        //     let x_r = x.0;
        //     let y_r = y.0;
        //
        //  x
        //
        // }).collect(

        todo!()
    }
}