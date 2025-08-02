use wasm_bindgen::prelude::*;

mod animations;
mod objects;
mod rendering;
mod scenes;

static mut CAMERA_POS: rendering::coordinates::Coordinates = rendering::coordinates::Coordinates {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

const CAMERA_SENSITIVITY: f64 = 10.0;

#[allow(static_mut_refs)]
#[wasm_bindgen]
pub fn next_frame(
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
        pos: rendering::coordinates::Coordinates {
            x: cube_left,
            y: cube_top,
            z: 0.0,
        },

        width: cube_size as u32,
        height: cube_size as u32,
        depth: cube_size as u32,

        color: rendering::colors::RgbaColor {
            r: 30,
            g: 20,
            b: 120,
            a: 255,
        },
    };

    let light = objects::light_source::LightSource {
        x: 300,
        y: 150,
        z: 0,

        intensity: 400,

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
            .move_camera(CAMERA_POS.clone());

        scene.generate(&canvas).data
    }
}

#[wasm_bindgen]
pub fn camera_backwards() {
    unsafe {
        CAMERA_POS.z -= CAMERA_SENSITIVITY;
    }
}

#[wasm_bindgen]
pub fn camera_forwards() {
    unsafe {
        CAMERA_POS.z += CAMERA_SENSITIVITY;
    }
}

#[wasm_bindgen]
pub fn camera_left() {
    unsafe {
        CAMERA_POS.x -= CAMERA_SENSITIVITY;
    }
}

#[wasm_bindgen]
pub fn camera_right() {
    unsafe {
        CAMERA_POS.x += CAMERA_SENSITIVITY;
    }
}

#[wasm_bindgen]
pub fn camera_up() {
    unsafe {
        CAMERA_POS.y -= CAMERA_SENSITIVITY;
    }
}

#[wasm_bindgen]
pub fn camera_down() {
    unsafe {
        CAMERA_POS.y += CAMERA_SENSITIVITY;
    }
}
