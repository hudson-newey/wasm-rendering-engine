use crate::positioning;

#[derive(Clone)]
pub struct Camera {
    pub pos: positioning::coordinates::Coordinates,

    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}

impl Camera {
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

    pub fn set_roll(&mut self, value: f64) {
        if value < 0.0 || value > 360.0 {
            self.roll = value % 360.0;
            return;
        }

        self.roll = value;
    }
}
