use std::f64::consts::E;

use crate::{objects, positioning, rendering};

pub struct LightSource {
    pub pos: positioning::coordinates::Coordinates,

    // Decay describes the "half life" of the color.
    // E.g. A decay of 100 means that after 100 pixels, the light source
    // will be half as bright.
    pub decay: f64,
}

impl objects::drawable::Drawable for LightSource {
    fn draw(
        self: &Self,
        image: &mut rendering::image::ImageData,
        camera: &objects::camera::Camera,
    ) {
        image.for_each_pixel(
            |_| true,
            |pixel| {
                let dx = (pixel.x as f64 - self.pos.x).abs();
                let dy = (pixel.y as f64 - self.pos.y).abs();
                let distance = ((dx * dx) + (dy * dy)).sqrt();

                // I use exponential decay for brightness to test this object.
                // However, we should make it so that this only collides with
                // other objects.
                let pixel_luminance = 1.0 * E.powf(-self.decay * distance);

                pixel.color.clone().lighten(pixel_luminance as f32)
            },
        );
    }
}
