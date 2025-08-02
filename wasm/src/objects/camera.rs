use crate::rendering;

pub struct Camera {
    pub pos: rendering::coordinates::Coordinates,

    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}
