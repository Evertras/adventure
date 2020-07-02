use super::super::{components, resources::game_map::{GameMap, TileProperties}};
use specs::{Read, ReadStorage, System};

pub struct MapSync;

impl<'a> System<'a> for MapSync {
    type SystemData = (
        specs::Entities<'a>,
        Read<'a, GameMap>,
        ReadStorage<'a, components::Position>,
    );

    fn run(&mut self, (entities, game_map, position): Self::SystemData) {
        let game_map = &game_map;

        for (entity, pos) in (&entities, &position).join() {
            game_map.add(&pos, entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, RunNow, World, WorldExt};

    #[test]
    fn adds_entities_with_positions_to_game_map() {
        let mut world = World::new();
        let map = GameMap::new();
        let pos = components::Position::new(1, 1);

        world.register::<components::Position>();

        let ent_player = world
            .create_entity()
            .with(pos.clone())
            .build();

        world.insert(map);

        let mut map_sync = MapSync;
        map_sync.run_now(&world);
        world.maintain();

        let map = world.get_mut::<GameMap>().unwrap();

        let entities = map.get_entities(&pos);

        match entities {
            None => panic!("Didn't get any entities back"),
            Some(entities) => {
                assert_eq!(entities.len(), 1);
            },
        };
    }

    #[test]
    fn blocks_tile_when_blocking_entity_exists() {
        let mut world = World::new();
        let map = GameMap::new();
        let pos = components::Position::new(1, 1);

        world.register::<components::Blocking>();
        world.register::<components::Position>();

        let ent_player = world
            .create_entity()
            .with(components::Blocking)
            .with(pos.clone())
            .build();

        world.insert(map);

        let mut map_sync = MapSync;
        map_sync.run_now(&world);
        world.maintain();

        let map = world.get_mut::<GameMap>().unwrap();

        assert!(map.tile_is(&pos, TileProperties::BLOCKED));
    }
}
