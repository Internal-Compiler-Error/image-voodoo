use super::*;
use itertools::iproduct;

impl CanvasImage {
    /// Remove `removal` pixels from the right side of the image
    pub fn crop_right(&self, removal: u32) -> CanvasImage {
        let old_h_size = self.horizontal_size();
        let v_size = self.vertical_size();

        // if they want to crop more than the image is wide, just return a blank image
        let cropped = old_h_size.checked_sub(removal).unwrap_or(0);

        let rgba = iproduct!(0..v_size, 0..cropped).flat_map(|(y, x)| {
            let rgba = self.rgba(x, y).unwrap();
            [rgba.0, rgba.1, rgba.2, rgba.3]
        });

        let buffer = Vec::from_iter(rgba);

        CanvasImage::from_vec_with_size(buffer, cropped, v_size)
    }

    /// Remove `removal` pixels from the bottom of the image
    pub fn crop_bottom(&self, removal: u32) -> CanvasImage {
        let h_size = self.horizontal_size();
        let old_v_size = self.vertical_size();

        let cropped = old_v_size.checked_sub(removal).unwrap_or(0);

        let rgba = iproduct!(0..cropped, 0..h_size).flat_map(|(y, x)| {
            let rgba = self.rgba(x, y).unwrap();
            [rgba.0, rgba.1, rgba.2, rgba.3]
        });

        let buffer = Vec::from_iter(rgba);

        CanvasImage::from_vec_with_size(buffer, h_size, cropped)
    }
}

#[wasm_bindgen]
pub fn crop_right(image: ImageData, removal: u32) -> ImageData {
    let image = CanvasImage::from_image_data(image);
    image.crop_right(removal).into()
}

#[wasm_bindgen]
pub fn crop_bottom(image: ImageData, removal: u32) -> ImageData {
    let image = CanvasImage::from_image_data(image);
    image.crop_bottom(removal).into()
}
