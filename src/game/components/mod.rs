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

        (x_diff * x_diff + y_diff * y_diff) as f32
    }

    fn angle_from_diff(x_diff: f32, y_diff: f32) -> f32 {
        let quadrant = y_diff.atan2(x_diff);

        if quadrant < 0. {
            quadrant + 2. * std::f32::consts::PI
        } else {
            quadrant
        }
    }

    pub fn theta(&self, to: &Position) -> f32 {
        let x_diff = (to.x - self.x) as f32;
        let y_diff = (to.y - self.y) as f32;

        Position::angle_from_diff(x_diff, y_diff)
    }

    pub fn visible_corner_thetas(&self, to: &Position) -> (f32, f32) {
        if self.x == to.x && self.y == to.y {
            return (0., 0.);
        }

        let offsets = if self.x == to.x {
            let y_offset = if self.y < to.y { -0.5 } else { 0.5 };

            ((-0.5, y_offset), (0.5, y_offset))
        } else if self.y == to.y {
            let x_offset = if self.x < to.x { -0.5 } else { 0.5 };

            ((x_offset, -0.5), (x_offset, 0.5))
        } else if self.x > to.x {
            if self.y > to.y {
                ((-0.5, 0.5), (0.5, -0.5))
            } else {
                ((0.5, 0.5), (-0.5, -0.5))
            }
        } else
        /*if self.x < to.x*/
        {
            if self.y > to.y {
                ((-0.5, -0.5), (0.5, 0.5))
            } else {
                ((-0.5, 0.5), (0.5, -0.5))
            }
        };

        let corner_a = offsets.0;
        let corner_b = offsets.1;

        let diff_x = (to.x - self.x) as f32;
        let diff_y = (to.y - self.y) as f32;

        let corner_a_diff_x = diff_x + corner_a.0;
        let corner_a_diff_y = diff_y + corner_a.1;

        let corner_b_diff_x = diff_x + corner_b.0;
        let corner_b_diff_y = diff_y + corner_b.1;

        (
            Position::angle_from_diff(corner_a_diff_x, corner_a_diff_y),
            Position::angle_from_diff(corner_b_diff_x, corner_b_diff_y),
        )
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

        check(2. * 2., start.distance_squared(&start.down().down()));
        check(1. + 1., start.distance_squared(&start.right().down()));
        check(
            1. + 4.,
            start.distance_squared(&start.right().down().down()),
        );
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

        check(3. * consts::FRAC_PI_2, start.theta(&up));
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

        check(3. * consts::FRAC_PI_4, start.theta(&diagonal));
    }

    #[test]
    fn theta_up_right_is_7_pi_over_4() {
        let start = Position::new(3, 4);
        let diagonal = start.right().up();

        check(7. * consts::FRAC_PI_4, start.theta(&diagonal));
    }

    #[test]
    fn theta_up_left_is_5_pi_over_4() {
        let start = Position::new(3, 4);
        let diagonal = start.left().up();

        check(5. * consts::FRAC_PI_4, start.theta(&diagonal));
    }

    fn check_corner_thetas(
        from: Position,
        to: Position,
        expect_min_theta: f32,
        expect_max_theta: f32,
    ) {
        let (theta_a, theta_b) = from.visible_corner_thetas(&to);

        let (min_theta, max_theta) = if theta_a > theta_b {
            (theta_b, theta_a)
        } else {
            (theta_a, theta_b)
        };

        if (expect_min_theta - min_theta).abs() > 1e-5 {
            panic!(
                "Min theta incorrect: expected {} but got {}",
                expect_min_theta, min_theta
            );
        }

        if (expect_max_theta - max_theta).abs() > 1e-5 {
            panic!(
                "Max theta incorrect: expected {} but got {}",
                expect_max_theta, max_theta
            );
        }
    }

    #[test]
    fn theta_corner_angles_for_right_adjacent_are_expected_values() {
        let start = Position::new(3, 4);
        let adj = start.right();

        check_corner_thetas(
            start,
            adj,
            std::f32::consts::FRAC_PI_4,
            7. * std::f32::consts::FRAC_PI_4,
        );
    }

    #[test]
    fn theta_corner_angles_for_left_adjacent_are_expected_values() {
        let start = Position::new(3, 4);
        let adj = start.left();

        check_corner_thetas(
            start,
            adj,
            3. * std::f32::consts::FRAC_PI_4,
            5. * std::f32::consts::FRAC_PI_4,
        );
    }

    #[test]
    fn theta_corner_angles_for_up_adjacent_are_expected_values() {
        let start = Position::new(3, 4);
        let adj = start.up();

        check_corner_thetas(
            start,
            adj,
            5. * std::f32::consts::FRAC_PI_4,
            7. * std::f32::consts::FRAC_PI_4,
        );
    }

    #[test]
    fn theta_corner_angles_for_down_adjacent_are_expected_values() {
        let start = Position::new(3, 4);
        let adj = start.down();

        check_corner_thetas(
            start,
            adj,
            std::f32::consts::FRAC_PI_4,
            3. * std::f32::consts::FRAC_PI_4,
        );
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
