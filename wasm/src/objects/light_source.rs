use crate::{objects, positioning, rendering};

pub struct LightSource {
    pub pos: positioning::coordinates::Coordinates,

    // Decay describes the "half life" of the color.
    // E.g. A decay of 100 means that after 100 pixels, the light source
    // will be half as bright.
    pub decay: f32,
}

impl objects::drawable::Drawable for LightSource {
    fn draw<'a>(
        self: &Self,
        image: &mut rendering::image::ImageData,
        camera: &objects::camera::Camera,
    ) {
        const LUMINANCE_MINIMUM: f32 = 0.01;
        let x_offset = self.pos.x + camera.pos.x;
        let y_offset = self.pos.y + camera.pos.y;

        image.for_each_pixel(
            |pixel| {
                let dx = (pixel.x as f32 - x_offset).abs();
                let dy = (pixel.y as f32 - y_offset).abs();
                let distance = ((dx * dx) + (dy * dy)).sqrt();

                // Note that this luminance function is extremely simple because
                // floating point powers are extremely slow.
                // let pixel_luminance = (-self.decay * distance).exp();
                let pixel_luminance = 1.0 - (self.decay * distance);

                // If the lightness amount gets less than 0.01, we say that the lighting
                // amount is "negligible".
                // At this point, we do not apply any transformations.
                let activate = pixel_luminance > LUMINANCE_MINIMUM;
                let passthrough_data = pixel_luminance;

                (activate, passthrough_data)
            },
            |pixel, passthrough| pixel.color.lighten(passthrough),
        );
    }
}
