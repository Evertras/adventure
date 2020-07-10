use super::super::{
    components,
    resources::game_map::{GameMap, TileProperties},
};
use specs::{ReadStorage, System, Write};

pub struct SyncGameMap;

impl<'a> System<'a> for SyncGameMap {
    type SystemData = (
        specs::Entities<'a>,
        Write<'a, GameMap>,
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::material::Material>,
        ReadStorage<'a, components::Shape>,
    );

    fn run(&mut self, (entities, mut game_map, positions, materials, shapes): Self::SystemData) {
        use specs::Join;

        // This is super inefficient and we should figure out how to do this only
        // for actual changes we care about, but for now simplicity wins
        game_map.clear_all();

        for (entity, pos, material, shape) in (&entities, &positions, &materials, &shapes).join() {
            game_map.add(&pos, entity.clone());

            if material.solid {
                match shape {
                    components::Shape::FullBlock => {
                        game_map.mark_tile(&pos, TileProperties::BLOCKED)
                    }
                    _ => (),
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, RunNow, World, WorldExt};

    fn build_world() -> specs::World {
        let mut world = World::new();
        let game_map = GameMap::new();

        world.insert(game_map);

        world.register::<components::Position>();
        world.register::<components::Moved>();
        world.register::<components::material::Material>();
        world.register::<components::Shape>();

        world
    }

    #[test]
    fn entity_without_solid_doesnt_block_tile() {
        let mut world = build_world();
        let pos = components::Position::new(3, -4);

        world
            .create_entity()
            .with(pos.clone())
            .with(components::material::smoke())
            .with(components::Shape::FullBlock)
            .build();

        let mut sync_game_map = SyncGameMap;
        sync_game_map.run_now(&world);
        world.maintain();

        let game_map = world.read_resource::<GameMap>();
        assert!(!game_map.tile_is(&pos, TileProperties::BLOCKED));
    }

    #[test]
    fn full_block_of_stone_marks_tile_as_blocked() {
        let mut world = build_world();
        let pos = components::Position::new(3, -4);

        world
            .create_entity()
            .with(pos.clone())
            .with(components::material::stone())
            .with(components::Shape::FullBlock)
            .build();

        let mut sync_game_map = SyncGameMap;
        sync_game_map.run_now(&world);
        world.maintain();

        let game_map = world.read_resource::<GameMap>();
        assert!(game_map.tile_is(&pos, TileProperties::BLOCKED));
    }

    #[test]
    fn stone_floor_not_blocked() {
        let mut world = build_world();
        let pos = components::Position::new(3, -4);

        world
            .create_entity()
            .with(pos.clone())
            .with(components::material::stone())
            .with(components::Shape::Floor)
            .build();

        let mut sync_game_map = SyncGameMap;
        sync_game_map.run_now(&world);
        world.maintain();

        let game_map = world.read_resource::<GameMap>();
        assert!(!game_map.tile_is(&pos, TileProperties::BLOCKED));
    }
}
