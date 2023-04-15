use super::*;
use itertools::iproduct;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FilterMode {
    Min,
    Max,
    Median,
}

impl CanvasImage {
    /// Filters the image using the given filter and distance
    pub fn filter(&self, filter: FilterMode, distance: u32) -> CanvasImage {
        // Given an iterator over the pixels within the distance, returns the filtered value
        let strategy: Box<dyn Fn(&mut dyn Iterator<Item = u8>) -> Option<u8>> = match filter {
            FilterMode::Min => Box::new(|iter| iter.min()),
            FilterMode::Max => Box::new(|iter| iter.max()),
            FilterMode::Median => Box::new(|iter| {
                let mut vec: Vec<_> = iter.collect();
                vec.sort_unstable();
                vec.get(vec.len() / 2).copied()
            }),
        };

        let distance = distance as isize;
        let r = |x, y| ReflectiveIndexedImage::g(self, x, y);
        let g = |x, y| ReflectiveIndexedImage::g(self, x, y);
        let b = |x, y| ReflectiveIndexedImage::g(self, x, y);
        let a = |x, y| ReflectiveIndexedImage::g(self, x, y);

        let width = self.horizontal_size() as isize;
        let height = self.vertical_size() as isize;

        // create the set that contains all x and y offsets that are within the distance
        let offsets = iproduct!(-distance..=distance).flat_map(|x_offset| {
            iproduct!(-(distance - x_offset.abs())..=(distance - x_offset.abs()))
                .map(move |y_offset| (x_offset, y_offset))
        });

        let rgba = iproduct!(0..height, 0..width)
            // map each pixel to a set of pixels that are within the distance
            .map(|(y, x)| {
                let mut r = offsets
                    .clone()
                    .map(|(y_offset, x_offset)| (y + y_offset, x + x_offset))
                    .map(|(y_prime, x_prime)| r(x_prime as i32, y_prime as i32));
                let r = strategy(&mut r);

                let mut g = offsets
                    .clone()
                    .map(|(y_offset, x_offset)| (y + y_offset, x + x_offset))
                    .map(|(y_prime, x_prime)| g(x_prime as i32, y_prime as i32));
                let g = strategy(&mut g);

                let mut b = offsets
                    .clone()
                    .map(|(y_offset, x_offset)| (y + y_offset, x + x_offset))
                    .map(|(y_prime, x_prime)| b(x_prime as i32, y_prime as i32));
                let b = strategy(&mut b);

                let a = a(x as i32, y as i32);

                [r.unwrap(), g.unwrap(), b.unwrap(), a]
            });

        let buffer = Vec::from_iter(rgba.flatten());

        CanvasImage::from_vec_with_size(buffer, self.horizontal_size(), self.vertical_size())
    }
}

#[wasm_bindgen]
pub fn filter(image: ImageData, distance: u32, filter: FilterMode) -> ImageData {
    let canvas_image = CanvasImage::from_image_data(image);
    let filtered = canvas_image.filter(filter, distance);
    filtered.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    #[test]
    fn min_filter() {
        // read the picture from file
        let image = image::open("meme.png").unwrap();

        // convert to RGBA
        let image = image.into_rgba8();

        // convert to CanvasImage
        let width = image.width();
        let height = image.height();
        let mut canvas_image = CanvasImage::from_vec_with_size(image.into_raw(), width, height);
        canvas_image.convert_to_greyscale();

        let filtered = canvas_image.filter(FilterMode::Min, 3);

        // convert to back to image and save
        let image: ImageBuffer<Rgba<u8>, &[u8]> = ImageBuffer::from_raw(
            filtered.horizontal_size(),
            filtered.vertical_size(),
            filtered.rgba_slice().clone(),
        )
        .unwrap();
        image.save("meme_min_filter.png").unwrap();
    }
}
