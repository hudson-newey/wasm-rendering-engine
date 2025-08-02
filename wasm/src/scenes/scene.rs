use crate::{objects::{self, drawable::Drawable}, rendering};

pub fn new_scene() -> Scene {
    let camera = objects::camera::Camera {
        pos: rendering::coordinates::Coordinates{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        pitch: 0.0,
        yaw: 0.0,
        roll: 0.0,
    };

   Scene { objects: vec![], camera }
}

pub struct Scene {
    camera: objects::camera::Camera,
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

    pub fn move_camera(mut self, pos: rendering::coordinates::Coordinates) -> Self {
        self.camera.pos = pos;
        self
    }
}
