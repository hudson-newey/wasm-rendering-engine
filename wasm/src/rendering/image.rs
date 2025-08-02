use crate::rendering::{self, colors};

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
        When: Fn(&rendering::pixel::Pixel) -> bool,
        Action: Fn(&colors::RgbaColor) -> colors::RgbaColor,
    {
        let mut result: Vec<RawValue> = Vec::with_capacity(self.data.len());

        for (index, raw_data) in self.data.chunks(4).enumerate() {
            let pixel = self.data_index_to_pixel(index, raw_data);

            if when(&pixel) {
                let color = action(&pixel.color);

                result.push(color.r);
                result.push(color.g);
                result.push(color.b);
                result.push(color.a);
            } else {
                result.extend_from_slice(raw_data);
            }
        }

        ImageData {
            data: result,
            canvas: self.canvas.clone(),
        }
    }

    fn data_index_to_pixel(&self, offset: usize, raw_data: &[u8]) -> rendering::pixel::Pixel {
        let x= offset as u32 % self.canvas.width;
        let y = offset as u32 / self.canvas.width;
        let color = image_data_to_rgba(raw_data);

        rendering::pixel::Pixel { x, y, color }
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
