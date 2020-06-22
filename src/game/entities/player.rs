use super::super::components;
use specs::{Builder, World, WorldExt};

pub fn create_in(world: &mut World) {
    world
        .create_entity()
        .with(components::Position { x: 0., y: 0. })
        .with(components::Draw {
            fg_r: 255,
            fg_g: 64,
            fg_b: 128,
            bg_r: 0,
            bg_g: 0,
            bg_b: 0,
            rune: '@',
        })
        .with(components::Player)
        .build();
}
