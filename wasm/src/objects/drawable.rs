use crate::{objects, rendering};

pub trait Drawable {
    fn draw(
        self: &Self,
        image: &rendering::image::ImageData,
        camera: &objects::camera::Camera,
    ) -> rendering::image::ImageData;
}
