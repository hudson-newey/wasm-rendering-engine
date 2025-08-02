use crate::{objects::{self, drawable::Drawable}, rendering};

pub fn new_scene(objects: Vec<Box<dyn Drawable>>) -> Scene {
    let camera = objects::camera::Camera {
        x: 0,
        y: 300,
        z: 0,
        pitch: 135,
        yaw: 0,
        roll: 0,
    };

   Scene { objects, camera }
}

pub struct Scene {
    objects: Vec<Box<dyn objects::drawable::Drawable>>,
    camera: objects::camera::Camera,
}

impl Scene {
    pub fn generate(self, canvas: &rendering::canvas::Canvas) -> rendering::image::ImageData {
        let mut image = rendering::image::from_canvas(canvas.clone());

        // before rendering anything, we want to black out the entire canvas
        image = image.for_each_pixel(
            |_| true,
            |_| rendering::colors::RgbaColor {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
        );

        for object in self.objects  {
            image = object.draw(&image, &self.camera);
        }

        image
    }
}
