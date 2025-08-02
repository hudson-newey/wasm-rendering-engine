use crate::{
    objects, positioning,
    rendering::{colors, image::ImageData},
};

pub struct Cube {
    pub pos: positioning::coordinates::Coordinates,

    pub width: f64,
    pub height: f64,
    pub depth: f64,

    pub color: colors::RgbaColor,
}

impl objects::drawable::Drawable for Cube {
    fn draw(&self, image: &ImageData, camera: &objects::camera::Camera) -> ImageData {
        let z_distance = self.pos.z - camera.pos.z;

        let top_pos = self.pos.y + z_distance - camera.pos.y;
        let bottom_pos = self.pos.y + (self.height as f64) - z_distance - camera.pos.y;

        let left_pos = self.pos.x + z_distance - camera.pos.x;
        let right_pos = self.pos.x + (self.width as f64) - z_distance - camera.pos.x;

        image.for_each_pixel(
            |pixel| {
                (pixel.y as f64) > top_pos
                    && (pixel.y as f64) < bottom_pos
                    && (pixel.x as f64) > left_pos
                    && (pixel.x as f64) < right_pos
            },
            |_| self.color.clone(),
        )
    }
}
