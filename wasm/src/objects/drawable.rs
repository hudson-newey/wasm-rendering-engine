use crate::{objects, rendering};

pub trait Drawable {
    fn draw(
        self: &Self,
        image: &mut rendering::image::ImageData,
        camera: &objects::camera::Camera,
    );
}
