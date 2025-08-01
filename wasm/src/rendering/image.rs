use crate::rendering::{colors, pixel};

// A single pixel value.
// E.g. A red, green, blue, or alpha value
pub type RawValue = u8;

pub struct ImageData {
    pub data: Vec<RawValue>,
    pub width: u32,
    pub height: u32,
}

impl ImageData {
    pub fn to_pixels(&self) -> Vec<pixel::Pixel> {
        let channels = 4;
        let mut result: Vec<pixel::Pixel> = Vec::with_capacity(self.data.len() / channels);

        for (index, pixel) in self.data.chunks(channels).enumerate() {
            let color = image_data_to_rgba(pixel);
            let pixel_offset = ((index / 4) as f32).floor() as u32;

            let y = (pixel_offset as f32 / self.width as f32).floor() as u32;
            let x = (pixel_offset as f32 % self.width as f32).floor() as u32;

            let pixel = pixel::Pixel { x, y, color };

            result.push(pixel);
        }

        result
    }

    // A really slow helper to conditionally apply a function to every pixel.
    pub fn for_each_pixel<When, Action>(
        &self,
        when: When,
        action: Action,
    ) -> ImageData
    where
        When: Fn(pixel::PixelOffset) -> bool,
        Action: Fn(&colors::RgbaColor) -> colors::RgbaColor,
    {
        let mut result: Vec<RawValue> = Vec::with_capacity(self.data.len());

        for (index, pixel) in self.data.chunks(4).enumerate() {
            if when(index as u32) {
                let color = action(&image_data_to_rgba(pixel));

                result.push(color.r);
                result.push(color.g);
                result.push(color.b);
                result.push(color.a);
            } else {
                result.extend_from_slice(pixel);
            }
        }

        ImageData {
            data: result,
            width: self.width,
            height: self.height,
        }
    }
}

fn image_data_to_rgba(buffer: &[u8]) -> colors::RgbaColor {
    colors::RgbaColor {
        r: buffer[0],
        g: buffer[1],
        b: buffer[2],
        a: buffer[3],
    }
}
