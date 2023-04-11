use itertools::iproduct;
use super::*;


impl CanvasImage {
    #[allow(dead_code, unused)]
    /// For each pixel,
    pub fn apply_min_filer(&self, distance: u32) -> CanvasImage {
        // assert!(distance.is_sign_positive(), "Distance must be positive");
        let distance = distance as isize;
        let r = |x, y| ReflectiveIndexedImage::g(self, x, y);
        let g = |x, y| ReflectiveIndexedImage::g(self, x, y);
        let b = |x, y| ReflectiveIndexedImage::g(self, x, y);
        let a = |x, y| ReflectiveIndexedImage::g(self, x, y);

        let width = self.horizontal_size() as isize;
        let height = self.vertical_size() as isize;
        let jump = distance as isize;


        // create the set that contains all x and y offsets that are within the distance
        let offsets =
            iproduct!(-distance..=distance)
                .flat_map(|x_offset| {
                    iproduct!(-(distance - x_offset.abs())..=(distance - x_offset.abs()))
                        .map(move |y_offset| (x_offset, y_offset))
                });


        let rgba = iproduct!(0..height, 0..width)
            // map each pixel to a set of pixels that are within the distance
            .map(|(y, x)| {
                let r_min = offsets.clone()
                    .map(|(y_offset, x_offset)| (y + y_offset, x + x_offset))
                    .map(|(y_prime, x_prime)| {
                        r(x_prime as i32, y_prime as i32)
                    }).min();

                let g_min = offsets.clone()
                    .map(|(y_offset, x_offset)| (y + y_offset, x + x_offset))
                    .map(|(y_prime, x_prime)| {
                        g(x_prime as i32, y_prime as i32)
                    }).min();

                let b_min = offsets.clone()
                    .map(|(y_offset, x_offset)| (y + y_offset, x + x_offset))
                    .map(|(y_prime, x_prime)| {
                        b(x_prime as i32, y_prime as i32)
                    }).min();

                let a = a(x as i32, y as i32);

                [r_min.unwrap(), g_min.unwrap(), b_min.unwrap(), 255]
            });

        let buffer = Vec::from_iter(rgba.flatten());

        CanvasImage::from_vec_with_size(buffer, self.horizontal_size(), self.vertical_size())
    }
}

#[cfg(test)]
mod tests {
    use image::{ImageBuffer, Rgba};
    use super::*;

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

        let filtered = canvas_image.apply_min_filer(3);

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