use super::super::{components, resources};
use specs::{Builder, World, WorldExt};

pub fn create_in(world: &mut World, pos: components::Position) -> specs::Entity {
    let ent = world
        .create_entity()
        .with(pos)
        .with(components::Sprite {
            fg_r: 255,
            fg_g: 64,
            fg_b: 128,
            bg_r: 0,
            bg_g: 0,
            bg_b: 0,
            layer: components::DL_ENTITY,
            rune: '@',
        })
        .with(components::Player)
        .with(components::material::flesh())
        .with(components::Shape::Medium)
        .build();

    world.insert(resources::Player { ent });

    ent
}
