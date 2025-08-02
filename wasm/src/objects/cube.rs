use crate::rendering::{colors, image::ImageData, pixel};

pub struct Cube {
    pub x: pixel::PixelOffset,
    pub y: pixel::PixelOffset,
    pub z: pixel::PixelOffset,

    pub width: pixel::PixelOffset,
    pub height: pixel::PixelOffset,
    pub depth: pixel::PixelOffset,

    pub color: colors::RgbaColor,
}

impl Cube {
    pub fn draw(&self, image: &ImageData) -> ImageData {
        image.for_each_pixel(
            // |index| index > (self.y * self.width) && index < ((self.y + self.height) * self.width) && index % self.width < self.x,
            |index| {
                index > (self.y * image.canvas.width)
                    && index < ((self.y + self.height) * image.canvas.width)
                    && index % image.canvas.width > self.x
                    && index % image.canvas.width < (self.x + self.width)
            },
            |_| self.color.clone(),
        )
    }
}
