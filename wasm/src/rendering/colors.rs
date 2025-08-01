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
