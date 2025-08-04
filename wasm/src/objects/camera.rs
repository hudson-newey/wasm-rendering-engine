use crate::positioning;

#[derive(Clone)]
pub struct Camera {
    pub pos: positioning::coordinates::Cartesian,
    pub facing: positioning::facing::Facing,
}
