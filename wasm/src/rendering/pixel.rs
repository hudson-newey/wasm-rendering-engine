use crate::rendering::colors::RgbaColor;

// A pixel offset that if dereferenced into raw values contains four rgba raw
// values.
pub type PixelOffset = u32;

pub struct Pixel {
    pub x: PixelOffset,
    pub y: PixelOffset,
    pub color: RgbaColor,
}
