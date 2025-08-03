use crate::{
    objects::{self, drawable::Drawable},
    positioning, rendering,
};

pub fn new_scene() -> Scene<'static> {
    let camera = Box::new(&objects::camera::Camera {
        pos: positioning::coordinates::Coordinates {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        facing: positioning::facing::ZEROED_FACING,
    });

    Scene {
        objects: vec![],
        camera: camera,
    }
}

pub struct Scene<'lifetime> {
    camera: Box<&'lifetime objects::camera::Camera>,
    objects: Vec<Box<dyn objects::drawable::Drawable>>,
}

impl Scene<'static> {
    pub fn generate(self, canvas: &rendering::canvas::Canvas) -> rendering::image::ImageData {
        let mut image = rendering::image::from_canvas(canvas.clone());

        // before rendering anything, we want to black out the entire canvas
        image.for_each_pixel(
            |_| (true, false),
            |_,_| &rendering::colors::RgbaColor {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
        );

        for object in self.objects {
            object.draw(&mut image, &self.camera);
        }

        image
    }

    pub fn add_object<T: Drawable + 'static>(mut self, object: T) -> Self {
        let boxed_object = Box::new(object);
        self.objects.push(boxed_object);
        self
    }

    pub fn move_camera(mut self, new_camera: &'static objects::camera::Camera) -> Self {
        self.camera = Box::new(new_camera);
        self
    }
}
