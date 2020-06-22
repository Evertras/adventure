use specs::{Component, DenseVecStorage, NullStorage, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Draw {
    pub fg_r: u8,
    pub fg_g: u8,
    pub fg_b: u8,

    pub bg_r: u8,
    pub bg_g: u8,
    pub bg_b: u8,

    pub rune: char,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;
