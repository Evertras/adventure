use specs::{Component, HashMapStorage, NullStorage, VecStorage};

pub mod material;

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

    pub fn distance_squared(&self, to: &Position) -> f32 {
        let x_diff = to.x - self.x;
        let y_diff = to.y - self.y;

        (x_diff*x_diff + y_diff*y_diff) as f32
    }

    pub fn theta(&self, to: &Position) -> f32 {
        let x_diff = (to.x - self.x) as f32;
        let y_diff = (to.y - self.y) as f32;

        let quadrant = y_diff.atan2(x_diff);

        if quadrant < 0. {
            quadrant + 2.*std::f32::consts::PI
        } else {
            quadrant
        }
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;
    use std::f32::consts;

    fn check(target: f32, result: f32) {
        let diff = (result - target).abs();

        if diff > 1e-5 {
            assert_eq!(target, result);
        }
    }

    #[test]
    fn cardinal_directions_have_distance_one() {
        let start = Position::new(7, -31);

        check(1., start.distance_squared(&start.right()));
        check(1., start.distance_squared(&start.left()));
        check(1., start.distance_squared(&start.up()));
        check(1., start.distance_squared(&start.down()));

        check(2.*2., start.distance_squared(&start.down().down()));
        check(1.+1., start.distance_squared(&start.right().down()));
        check(1.+4., start.distance_squared(&start.right().down().down()));
        check(0., start.distance_squared(&start.up().down()));
    }

    #[test]
    fn theta_right_is_zero() {
        let start = Position::new(-30, 1);
        let right = start.right();

        check(0., start.theta(&right));
    }

    #[test]
    fn theta_down_is_pi_over_2() {
        let start = Position::new(3, 4);
        let down = start.down();

        check(consts::FRAC_PI_2, start.theta(&down));
    }

    #[test]
    fn theta_up_is_3_pi_over_2() {
        let start = Position::new(3, 4);
        let up = start.up();

        check(3.*consts::FRAC_PI_2, start.theta(&up));
    }

    #[test]
    fn theta_left_is_pi() {
        let start = Position::new(3, 4);
        let left = start.left();

        check(consts::PI, start.theta(&left));
    }

    #[test]
    fn theta_down_right_is_pi_over_4() {
        let start = Position::new(3, 4);
        let diagonal = start.right().down();

        check(consts::FRAC_PI_4, start.theta(&diagonal));
    }

    #[test]
    fn theta_down_left_is_3_pi_over_4() {
        let start = Position::new(3, 4);
        let diagonal = start.left().down();

        check(3.*consts::FRAC_PI_4, start.theta(&diagonal));
    }

    #[test]
    fn theta_up_right_is_7_pi_over_4() {
        let start = Position::new(3, 4);
        let diagonal = start.right().up();

        check(7.*consts::FRAC_PI_4, start.theta(&diagonal));
    }

    #[test]
    fn theta_up_left_is_5_pi_over_4() {
        let start = Position::new(3, 4);
        let diagonal = start.left().up();

        check(5.*consts::FRAC_PI_4, start.theta(&diagonal));
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
pub struct Sprite {
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

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Visible;

#[derive(Clone, Component, Debug, PartialEq, Eq)]
#[storage(VecStorage)]
pub enum Shape {
    Floor,
    Medium,
    FullBlock,
}
