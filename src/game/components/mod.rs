use specs::prelude::*;

#[derive(Clone, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

