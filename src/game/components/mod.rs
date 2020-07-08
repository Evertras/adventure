use specs::{Component, HashMapStorage, NullStorage, VecStorage};

#[derive(Clone, Component, Debug, PartialEq, Eq, Hash)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn set(&mut self, to: &Position) {
        self.x = to.x;
        self.y = to.y;
    }

    pub fn up(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Clone, Component, Debug, PartialEq, Eq, Hash)]
#[storage(HashMapStorage)]
pub struct Moved {
    pub from: Position,
    pub to: Position,
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
