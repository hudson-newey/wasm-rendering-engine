use crate::rendering;

type IntensityRed = u8;
type IntensityGreen = u8;
type IntensityBlue = u8;
type IntensityAlpha = u8;

#[derive(Clone)]
pub struct RgbaColor {
    pub r: IntensityRed,
    pub g: IntensityGreen,
    pub b: IntensityBlue,
    pub a: IntensityAlpha,
}

impl RgbaColor {
    pub fn to_image_data(&self) -> [rendering::image::RawValue; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
