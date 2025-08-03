use rayon::prelude::*;
use crate::rendering::{self, colors};

// A single pixel value.
// E.g. A red, green, blue, or alpha value
pub type RawValue = u8;

pub fn from_canvas(canvas: rendering::canvas::Canvas) -> ImageData {
    let data: Vec<RawValue> = (0..(canvas.width * canvas.height * 4)).map(|_| 0).collect();
    ImageData {
        data,
        canvas: canvas.clone(),
    }
}

#[derive(Clone)]
pub struct ImageData {
    pub data: Vec<RawValue>,
    pub canvas: rendering::canvas::Canvas,
}

impl ImageData {
    pub fn for_each_pixel<'passthrough, When, Action, DataPassthrough>(
        &mut self,
        when: When,
        action: Action,
    ) where
        When: Fn(&rendering::pixel::Pixel) -> (bool, DataPassthrough) + Sync + Send,
        Action: Fn(&mut rendering::pixel::Pixel, DataPassthrough) -> &colors::RgbaColor + Sync + Send,
    {
        const PIXEL_BYTE_SIZE: usize = 4;

        self.data
            .par_chunks_mut(PIXEL_BYTE_SIZE)
            .enumerate()
            .for_each(|(index, raw_data)| {
                let mut pixel = data_index_to_pixel(index, self.canvas.width, &raw_data);

                let (activate, data_passthrough) = when(&pixel);
                if activate {
                    let color = action(&mut pixel, data_passthrough);
                    let mut color_data = color.to_image_data();

                    raw_data.swap_with_slice(&mut color_data);
                }
            });
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: colors::RgbaColor) {
        let index = ((y * self.canvas.width) + x) as usize;
        self.data[index..index + 3].copy_from_slice(&color.to_image_data());
    }
}

fn data_index_to_pixel(
    offset: usize,
    canvas_width: u32,
    raw_data: &[u8],
) -> rendering::pixel::Pixel {
    let x = offset as u32 % canvas_width;
    let y = offset as u32 / canvas_width;
    let color = image_data_to_rgba(raw_data);

    rendering::pixel::Pixel { x, y, color }
}

fn image_data_to_rgba(buffer: &[u8]) -> colors::RgbaColor {
    colors::RgbaColor {
        r: buffer[0],
        g: buffer[1],
        b: buffer[2],
        a: buffer[3],
    }
}
