use wasm_bindgen::prelude::*;

struct RgbaColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen]
pub fn next_frame(current_frame: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let background: Vec<u8> = for_each_pixel(
        current_frame,
        |_| true,
        |_| RgbaColor{ r: 0, g: 0, b: 0, a: 255 },
    );

    let cube_size: u32 = 600;
    let midpoint_left: u32 = width / 2;
    let midpoint_top: u32 = height / 2;

    let cube_left: u32 = midpoint_left - (cube_size / 2);
    let cube_top: u32 = midpoint_top - (cube_size / 2);

    let square: Vec<u8> = draw_surface(
        background,
        RgbaColor {
            r: 0,
            g: 0,
            b: 200,
            a: 200,
        },
        cube_left,
        cube_size,
        cube_top,
        cube_size,
        0,
        0,
    );

    return square;
}

fn pixel_to_rgba(buffer: &[u8]) -> RgbaColor {
    RgbaColor { r: buffer[0], g: buffer[1], b: buffer[2], a: buffer[3] }
}

// A really slow helper to conditionally apply a function to every pixel.
fn for_each_pixel<When, Action>(data: Vec<u8>, when: When, action: Action) -> Vec<u8>
where
    When: Fn(usize) -> bool,
    Action: Fn(&RgbaColor) -> RgbaColor,
{
    let mut result: Vec<u8> = Vec::with_capacity(data.len());

    for (index, pixel) in data.chunks(4).enumerate() {
        if when(index) {
            let color: RgbaColor = action(
                &pixel_to_rgba(pixel)
            );

            result.push(color.r);
            result.push(color.g);
            result.push(color.b);
            result.push(color.a);
        } else {
            result.extend_from_slice(pixel);
        }
    }

    result
}

fn draw_surface(
    data: Vec<u8>,
    color: RgbaColor,
    x0: u32,
    x1: u32,
    y0: u32,
    y1: u32,
    z0: u32,
    z1: u32,
) -> Vec<u8> {
    return data;
}
