use crate::{objects, positioning, rendering};

pub struct LightSource {
    pub pos: positioning::coordinates::Coordinates,

    // Intensity describes the "half life" of the color.
    // E.g. An intensity of 100 means that after 100 pixels, the light source
    // will be half as bright.
    pub intensity: f64,

    pub color: rendering::colors::RgbaColor,
}

impl objects::drawable::Drawable for LightSource {
    fn draw(
        self: &Self,
        image: &rendering::image::ImageData,
        camera: &objects::camera::Camera,
    ) -> rendering::image::ImageData {
        image.for_each_pixel(|_| false, |_| self.color.clone())
    }
}
