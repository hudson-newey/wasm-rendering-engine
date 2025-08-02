use crate::positioning;

#[derive(Clone)]
pub struct Camera {
    pub pos: positioning::coordinates::Coordinates,
    pub facing: positioning::facing::Facing,
}
