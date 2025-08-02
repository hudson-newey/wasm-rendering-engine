use crate::{
    objects,
    rendering::{self, colors, image::ImageData, pixel},
};

pub struct Cube {
    pub pos: rendering::coordinates::Coordinates,

    pub width: pixel::PixelOffset,
    pub height: pixel::PixelOffset,
    pub depth: pixel::PixelOffset,

    pub color: colors::RgbaColor,
}

impl objects::drawable::Drawable for Cube {
    fn draw(&self, image: &ImageData, camera: &objects::camera::Camera) -> ImageData {
        let distance_from_camera = self.pos.z - camera.pos.z;

        image.for_each_pixel(
            |index| {
                index > (self.pos.y as u32 * image.canvas.width)
                    && index < ((self.pos.y as u32 + self.height) * image.canvas.width)
                    && index % image.canvas.width > self.pos.x as u32
                    && index % image.canvas.width < (self.pos.x as u32 + self.width)
            },
            |_| self.color.clone(),
        )
    }
}
