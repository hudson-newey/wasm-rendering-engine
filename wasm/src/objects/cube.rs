use crate::{
    objects, positioning,
    rendering::{colors, image::ImageData, pixel::PixelOffset},
};

pub struct Cube<'lifetime> {
    pub pos: positioning::coordinates::Cartesian,
    pub facing: positioning::facing::Facing,

    pub width: f32,
    pub height: f32,
    pub depth: f32,

    pub bg_color: &'lifetime colors::RgbaColor,

    pub line_color: &'lifetime colors::RgbaColor,
    pub line_width: PixelOffset,
}

impl objects::drawable::Drawable for Cube<'static> {
    fn draw(&self, image: &mut ImageData, camera: &objects::camera::Camera) {
        let z_distance = self.pos.z - camera.pos.z;

        let y_scale = z_distance;
        let top_pos = self.pos.y - camera.pos.y + y_scale;
        let bottom_pos = self.pos.y + (self.height as f32) - camera.pos.y - y_scale;

        let x_scale = z_distance;
        let left_pos = self.pos.x - camera.pos.x + x_scale;
        let right_pos = self.pos.x + (self.width as f32) - camera.pos.x - x_scale;

        image.for_each_pixel(
            |pixel| {
                let front_face = (pixel.y as f32) >= top_pos
                    && (pixel.y as f32) <= bottom_pos
                    && (pixel.x as f32) >= left_pos
                    && (pixel.x as f32) <= right_pos;

                let top_face = false;

                return (front_face || top_face, false);
            },
            |pixel, _| {
                let is_edge = (pixel.y as f32) < top_pos + (self.line_width as f32)
                    || (pixel.y as f32) > bottom_pos - self.line_width as f32
                    || (pixel.x as f32) < left_pos + self.line_width as f32
                    || (pixel.x as f32) > right_pos - self.line_width as f32;

                if is_edge {
                    &self.line_color
                } else {
                    &self.bg_color
                }
            },
        );
    }
}
