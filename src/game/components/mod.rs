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

#[derive(Clone, Debug)]
pub struct Draw {
    pub fg_r: u8,
    pub fg_g: u8,
    pub fg_b: u8,

    pub bg_r: u8,
    pub bg_g: u8,
    pub bg_b: u8,

    pub rune: char,
}

impl Component for Draw {
    type Storage = VecStorage<Self>;
}
