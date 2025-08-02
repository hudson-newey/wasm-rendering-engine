use wasm_bindgen::prelude::*;

mod objects;
mod rendering;
mod scenes;
mod animations;

static mut CAMERA_POS: f64 = 0.0;
static mut MOVING_IN: bool = false;

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

    let mut scene = scenes::scene::new_scene()
        .add_object(cube)
        .add_object(light);

    unsafe {
        if CAMERA_POS > 360.0 {
            MOVING_IN = true;
        } else if CAMERA_POS < 100.0 {
            MOVING_IN = false;
        };

        CAMERA_POS = if MOVING_IN {
            CAMERA_POS + 1.0
        } else {
            CAMERA_POS - 1.0
        };

        scene.camera.pos.z = CAMERA_POS;
    }

    scene.generate(&canvas).data
}
