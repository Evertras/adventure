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
        ReadStorage<'a, components::Solid>,
    );

    fn run(&mut self, (entities, mut game_map, positions, solids): Self::SystemData) {
        use specs::Join;

        // This is super inefficient and we should figure out how to do this only
        // for actual changes we care about, but for now simplicity wins
        game_map.clear_all();

        for (entity, pos, solid) in (&entities, &positions, (&solids).maybe()).join() {
            game_map.add(&pos, entity.clone());

            match solid {
                None => (),
                Some(_) => {
                    game_map.mark_tile(&pos, TileProperties::BLOCKED);
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, RunNow, World, WorldExt};

    #[test]
    fn entity_without_solid_doesnt_block_tile() {
        let mut world = World::new();
        let pos = components::Position::new(3, -4);
        let game_map = GameMap::new();

        world.register::<components::Position>();
        world.register::<components::Moved>();
        world.register::<components::Solid>();

        world.insert(game_map);

        world.create_entity().with(pos.clone()).build();

        let mut sync_game_map = SyncGameMap;
        sync_game_map.run_now(&world);
        world.maintain();

        let game_map = world.read_resource::<GameMap>();
        assert!(!game_map.tile_is(&pos, TileProperties::BLOCKED));
    }

    #[test]
    fn solid_entity_marks_tile_as_blocked() {
        let mut world = World::new();
        let pos = components::Position::new(3, -4);
        let game_map = GameMap::new();

        world.register::<components::Position>();
        world.register::<components::Moved>();
        world.register::<components::Solid>();

        world.insert(game_map);

        world
            .create_entity()
            .with(pos.clone())
            .with(components::Solid)
            .build();

        let mut sync_game_map = SyncGameMap;
        sync_game_map.run_now(&world);
        world.maintain();

        let game_map = world.read_resource::<GameMap>();
        assert!(game_map.tile_is(&pos, TileProperties::BLOCKED));
    }
}
