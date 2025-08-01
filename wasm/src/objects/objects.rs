use crate::rendering::image::ImageData;

pub trait Drawable {
    fn draw(&self, image: &ImageData) -> ImageData;
}
