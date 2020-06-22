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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DrawLayer(u8);

pub const DL_FLOOR: DrawLayer = DrawLayer(0);
pub const DL_WALLS: DrawLayer = DrawLayer(5);
pub const DL_ENTITY: DrawLayer = DrawLayer(100);

#[derive(Clone, Component, Debug, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Draw {
    pub fg_r: u8,
    pub fg_g: u8,
    pub fg_b: u8,

    pub bg_r: u8,
    pub bg_g: u8,
    pub bg_b: u8,

    pub layer: DrawLayer,

    pub rune: char,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;
