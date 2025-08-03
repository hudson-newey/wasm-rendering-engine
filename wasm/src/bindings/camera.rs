use wasm_bindgen::prelude::*;

use crate::{objects, positioning::{self, facing::ZEROED_FACING}};

pub static mut CAMERA: objects::camera::Camera = objects::camera::Camera {
    pos: positioning::coordinates::Coordinates {
        x: 0.0,
        y: -100.0,
        z: 0.0,
    },
    facing: ZEROED_FACING,
};

const CAMERA_SENSITIVITY: f32 = 10.0;

#[wasm_bindgen]
pub fn camera_backwards(value: Option<f32>) {
    unsafe {
        CAMERA.pos.z -= value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[wasm_bindgen]
pub fn camera_forwards(value: Option<f32>) {
    unsafe {
        CAMERA.pos.z += value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[wasm_bindgen]
pub fn camera_left(value: Option<f32>) {
    unsafe {
        CAMERA.pos.x -= value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[wasm_bindgen]
pub fn camera_right(value: Option<f32>) {
    unsafe {
        CAMERA.pos.x += value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[wasm_bindgen]
pub fn camera_up(value: Option<f32>) {
    unsafe {
        CAMERA.pos.y -= value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[wasm_bindgen]
pub fn camera_down(value: Option<f32>) {
    unsafe {
        CAMERA.pos.y += value.unwrap_or(CAMERA_SENSITIVITY);
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_left(value: Option<f32>) {
    unsafe {
        CAMERA.facing.add_yaw(-value.unwrap_or(CAMERA_SENSITIVITY));
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_right(value: Option<f32>) {
    unsafe {
        CAMERA.facing.add_yaw(value.unwrap_or(CAMERA_SENSITIVITY));
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_up(value: Option<f32>) {
    unsafe {
        CAMERA.facing.add_pitch(-value.unwrap_or(CAMERA_SENSITIVITY));
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_down(value: Option<f32>) {
    unsafe {
        CAMERA.facing.add_pitch(value.unwrap_or(CAMERA_SENSITIVITY));
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_anticlockwise(value: Option<f32>) {
    unsafe {
        CAMERA.facing.add_roll(-value.unwrap_or(CAMERA_SENSITIVITY));
    }
}

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn camera_rotate_clockwise(value: Option<f32>) {
    unsafe {
        CAMERA.facing.add_roll(value.unwrap_or(CAMERA_SENSITIVITY));
    }
}
