// use std::simd::num::{SimdFloat, SimdUint};
// use std::simd::{f32x4, u8x4};

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
    pub fn lighten(&mut self, amount: f32) -> &Self {
        let multiplier = 1.0 + amount;

        self.r = (self.r as f32 * multiplier) as u8;
        self.g = (self.g as f32 * multiplier) as u8;
        self.b = (self.b as f32 * multiplier) as u8;
        self

        // After benchmarking, I found that SIMD was actually slower in this
        // case.
        // Therefore, I have disabled it.

        // let multiplier = f32x4::splat(1.0 + amount);

        // let rgb = u8x4::from_array([self.r, self.g, self.b, 0]);
        // let rgb_f32 = rgb.cast::<f32>();
        // let result = rgb_f32 * multiplier;

        // let clamped = result
        //     .simd_max(f32x4::splat(0.0))
        //     .simd_min(f32x4::splat(255.0));

        // let final_rgb = clamped.cast::<u8>();

        // self.r = final_rgb[0];
        // self.g = final_rgb[1];
        // self.b = final_rgb[2];
        // self
    }
}
