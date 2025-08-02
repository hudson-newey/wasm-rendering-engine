use crate::positioning;

pub struct Camera {
    pub pos: positioning::coordinates::Coordinates,

    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}
