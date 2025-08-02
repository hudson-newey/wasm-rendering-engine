use crate::rendering::{self, colors, pixel};

// A single pixel value.
// E.g. A red, green, blue, or alpha value
pub type RawValue = u8;

pub fn from_canvas(canvas: rendering::canvas::Canvas) -> ImageData {
    let data: Vec<RawValue> = (0..(canvas.width * canvas.height * 4)).map(|_| 0).collect();
    ImageData { data, canvas: canvas.clone() }
}

pub struct ImageData {
    pub data: Vec<RawValue>,
    pub canvas: rendering::canvas::Canvas,
}

impl ImageData {
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
            canvas: self.canvas.clone(),
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
