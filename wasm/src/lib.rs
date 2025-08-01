use wasm_bindgen::prelude::*;

mod objects;
mod rendering;

#[wasm_bindgen]
pub fn next_frame(
    current_frame: Vec<rendering::image::RawValue>,
    width: rendering::pixel::PixelOffset,
    height: rendering::pixel::PixelOffset,
) -> Vec<rendering::image::RawValue> {
    let image_data = rendering::image::ImageData {
        data: current_frame,
        width,
        height,
    };

    let canvas = image_data.for_each_pixel(
        |_| true,
        |_| rendering::colors::RgbaColor {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
    );

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

    let cubed_canvas = cube.draw(&canvas);

    cubed_canvas.data
}
