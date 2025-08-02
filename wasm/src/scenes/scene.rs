use crate::{objects, rendering};

pub fn new_scene() -> Scene {
    let camera = objects::camera::Camera {
        x: 0,
        y: 0,
        z: 0,
        pitch: 0,
        yaw: 0,
        roll: 0,
    };

    Scene { objects: vec![], camera }
}

pub struct Scene {
    objects: Vec<objects::cube::Cube>,
    camera: objects::camera::Camera,
}

impl Scene {
    fn generate(canvas: rendering::canvas::Canvas) -> rendering::image::ImageData {
        rendering::image::ImageData {
            data: vec![],
            canvas,
        }
    }
}
