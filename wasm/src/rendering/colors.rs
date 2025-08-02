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

    // This function takes a percentage in the form of a decimal.
    // e.g. 0.2 will increase the current tiles luminance by 20%
    // I have chosen to use 0 values as a no op because a lot of low level
    // luminance functions of x will approach y = 0.
    // I'm therefore hoping that consumers will not have to remember to add one
    // every time and can directly pass the result of their lim->0 function
    // into this method.
    pub fn lighten(&self, amount: f32) -> Self {
        let r = (self.r as f32) * (1.0 + amount);
        let g = (self.g as f32) * (1.0 + amount);
        let b = (self.b as f32) * (1.0 + amount);

        RgbaColor {
            r: r.clamp(0.0, 255.0) as u8,
            g: g.clamp(0.0, 255.0) as u8,
            b: b.clamp(0.0, 255.0) as u8,
            a: self.a,
        }
    }
}
