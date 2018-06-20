/// Two dimensional vector of usizes
#[derive(Clone)]
pub struct Vec2u {
    pub x: usize,
    pub y: usize,
}

impl Vec2u {
    pub fn new(x: usize, y: usize) -> Self {
        Vec2u { x: x, y: y }
    }
}
