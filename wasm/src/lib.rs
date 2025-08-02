use wasm_bindgen::prelude::*;

mod objects;
mod rendering;
mod scenes;

#[wasm_bindgen]
pub fn next_frame(
    width: rendering::pixel::PixelOffset,
    height: rendering::pixel::PixelOffset,
) -> Vec<rendering::image::RawValue> {
    let canvas = rendering::canvas::Canvas { width, height };

    let cube_size: u32 = 600;
    let midpoint_left: u32 = width / 2;
    let midpoint_top: u32 = height / 2;

    let cube_left: u32 = midpoint_left - (cube_size / 2);
    let cube_top: u32 = midpoint_top - (cube_size / 2);

    let cube = objects::cube::Cube {
        x: cube_left,
        y: cube_top,
        z: 0,

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

    let scene = scenes::scene::new_scene(vec![Box::new(cube), Box::new(light)]);

    scene.generate(&canvas).data
}
