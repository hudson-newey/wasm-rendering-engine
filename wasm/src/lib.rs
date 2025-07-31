use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn next_frame(current_frame: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    return current_frame
        .iter()
        .enumerate()
        .map(|(index, value)| blackout(index, value))
        .collect();
}

fn is_alpha_channel(index: usize) -> bool {
    return index % 4 == 3;
}

fn blackout(index: usize, value: &u8) -> u8 {
    if is_alpha_channel(index) {
        return 255;
    }

    return 0;
}
