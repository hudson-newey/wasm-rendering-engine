use crate::{objects::{self, drawable::Drawable}, rendering};

pub fn new_scene() -> Scene {
    let camera = objects::camera::Camera {
        x: 0,
        y: 300,
        z: 0,
        pitch: 135,
        yaw: 0,
        roll: 0,
    };

   Scene { objects: vec![], camera }
}

pub struct Scene {
    pub camera: objects::camera::Camera,
    objects: Vec<Box<dyn objects::drawable::Drawable>>,
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

    pub fn add_object<T: Drawable + 'static>(mut self, object: T) -> Self {
        let boxed_object = Box::new(object);
        self.objects.push(boxed_object);
        self
    }
}
