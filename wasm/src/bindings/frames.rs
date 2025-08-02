use wasm_bindgen::prelude::*;

use crate::{bindings::camera, objects, positioning, rendering, scenes};

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
        facing: positioning::facing::ZEROED_FACING,

        width: cube_size,
        height: cube_size,
        depth: cube_size,

        bg_color: rendering::colors::RgbaColor {
            r: 30,
            g: 20,
            b: 120,
            a: 255,
        },

        line_color: rendering::colors::RgbaColor {
            r: 120,
            g: 20,
            b: 30,
            a: 255,
        },
        line_width: 3,
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
            .move_camera(camera::CAMERA.clone());

        scene.generate(&canvas).data
    }
}
