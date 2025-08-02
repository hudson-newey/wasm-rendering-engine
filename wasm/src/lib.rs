use wasm_bindgen::prelude::*;

mod animations;
mod objects;
mod rendering;
mod scenes;
mod positioning;

static mut CAMERA: objects::camera::Camera = objects::camera::Camera {
    pos: positioning::coordinates::Coordinates {
        x: 0.0,
        y: -100.0,
        z: 0.0,
    },
    pitch: 45.0,
    yaw: 0.0,
    roll: 0.0,
};

const CAMERA_SENSITIVITY: f64 = 10.0;

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn generate_frame(
    width: rendering::pixel::PixelOffset,
    height: rendering::pixel::PixelOffset,
) -> Vec<rendering::image::RawValue> {
    let canvas = rendering::canvas::Canvas { width, height };

    let cube_size = 600.0;
    let midpoint_left = width as f64 / 2.0;
    let midpoint_top = height as f64 / 2.0;

    let cube_left = midpoint_left - (cube_size / 2.0);
    let cube_top = midpoint_top - (cube_size / 2.0);

    let cube = objects::cube::Cube {
        pos: positioning::coordinates::Coordinates {
            x: cube_left,
            y: cube_top,
            z: 0.0,
        },

        width: cube_size,
        height: cube_size,
        depth: cube_size,

        color: rendering::colors::RgbaColor {
            r: 30,
            g: 20,
            b: 120,
            a: 255,
        },
    };

    let light = objects::light_source::LightSource {
        pos: positioning::coordinates::Coordinates {
            x: 300.0,
            y: 150.0,
            z: 0.0,
        },

        intensity: 400.0,

        color: rendering::colors::RgbaColor {
            r: 255,
            g: 255,
            b: 255,
            a: 100,
        },
    };

    // unsafe is needed because CAMERA_POS is static
    unsafe {
        let scene = scenes::scene::new_scene()
            .add_object(cube)
            .add_object(light)
            .move_camera(CAMERA.clone());

        scene.generate(&canvas).data
    }
}

#[wasm_bindgen]
pub fn camera_backwards(value: Option<f64>) {
    unsafe {
        CAMERA.pos.z -= value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[wasm_bindgen]
pub fn camera_forwards(value: Option<f64>) {
    unsafe {
        CAMERA.pos.z += value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[wasm_bindgen]
pub fn camera_left(value: Option<f64>) {
    unsafe {
        CAMERA.pos.x -= value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[wasm_bindgen]
pub fn camera_right(value: Option<f64>) {
    unsafe {
        CAMERA.pos.x += value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[wasm_bindgen]
pub fn camera_up(value: Option<f64>) {
    unsafe {
        CAMERA.pos.y -= value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[wasm_bindgen]
pub fn camera_down(value: Option<f64>) {
    unsafe {
        CAMERA.pos.y += value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_left(value: Option<f64>) {
    unsafe {
        CAMERA.set_yaw(CAMERA.yaw - value.unwrap_or(CAMERA_SENSITIVITY));
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_right(value: Option<f64>) {
    unsafe {
        CAMERA.set_yaw(CAMERA.yaw + value.unwrap_or(CAMERA_SENSITIVITY));
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_up(value: Option<f64>) {
    unsafe {
        CAMERA.set_pitch(CAMERA.pitch - value.unwrap_or(CAMERA_SENSITIVITY));
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_down(value: Option<f64>) {
    unsafe {
        CAMERA.set_pitch(CAMERA.pitch + value.unwrap_or(CAMERA_SENSITIVITY));
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_anticlockwise(value: Option<f64>) {
    unsafe {
        CAMERA.set_pitch(CAMERA.roll - value.unwrap_or(CAMERA_SENSITIVITY));
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_clockwise(value: Option<f64>) {
    unsafe {
        CAMERA.set_pitch(CAMERA.roll + value.unwrap_or(CAMERA_SENSITIVITY));
    }
}
