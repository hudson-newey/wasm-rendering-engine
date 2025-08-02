// Use this constant to construct a new facing.
// Because a lot of these values are quantized, I do not allow trust myself to
// manually modify the values.
// If you want to modify the values, you can use the builder pattern to change
// the values.
pub const ZEROED_FACING: Facing = Facing {
    pitch: 0.0,
    yaw: 0.0,
    roll: 0.0,
};

#[derive(Clone)]
pub struct Facing {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}

impl Facing {
    // Pitch, yaw, and roll are all quantized values and therefore should not
    // be directly set.
    // Using these setters will either limit the value to the maximum, or wrap
    // the value so that it appears nothing happens.
    //
    // Pitch will be limited to facing directly upwards and directly downwards.
    pub fn set_pitch(&mut self, value: f64) {
        if value < -180.0 {
            self.pitch = -180.0;
            return;
        } else if value > 180.0 {
            self.pitch = 180.0;
            return;
        }

        self.pitch = value;
    }

    pub fn add_pitch(&mut self, value: f64) {
        let current = self.clone().pitch;
        self.set_pitch(current + value);
    }

    pub fn set_yaw(&mut self, value: f64) {
        // We find the modulo of the value % 360 so that if the camera moves
        // really fast / far past 360, the camera will end up in the expected
        // yaw position.
        //
        // If we instead clamped the value to 0, there would be a stutter when
        // moving the camera past 360deg.
        if value < 0.0 || value > 360.0 {
            self.yaw = value % 360.0;
            return;
        }

        self.yaw = value;
    }

    pub fn add_yaw(&mut self, value: f64) {
        let current = self.clone().yaw;
        self.set_yaw(current + value);
    }

    pub fn set_roll(&mut self, value: f64) {
        if value < 0.0 || value > 360.0 {
            self.roll = value % 360.0;
            return;
        }

        self.roll = value;
    }

    pub fn add_roll(&mut self, value: f64) {
        let current = self.clone().roll;
        self.set_roll(current + value);
    }
}
