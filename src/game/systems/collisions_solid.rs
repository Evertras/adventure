use super::super::{
    components,
    resources::game_map::{GameMap, TileProperties},
};
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct CollisionsSolid;

impl<'a> System<'a> for CollisionsSolid {
    type SystemData = (
        WriteStorage<'a, components::Moved>,
        ReadStorage<'a, components::material::Material>,
        specs::Entities<'a>,
        Read<'a, GameMap>,
    );

    fn run(&mut self, (mut moved, materials, entities, game_map): Self::SystemData) {
        use specs::Join;

        let mut to_remove = Vec::new();

        for (mv, entity, material) in (&moved, &entities, &materials).join() {
            if material.solid && game_map.tile_is(&mv.to, TileProperties::BLOCKED) {
                to_remove.push(entity);
            }
        }

        for entity in to_remove {
            moved.remove(entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::resources::game_map::GameMap;
    use super::*;
    use specs::{Builder, RunNow, World, WorldExt};

    #[test]
    fn doesnt_affect_nonsolid_entities() {
        let mut world = World::new();
        let start = components::Position::new(3, -4);
        let target = components::Position::new(3, -5);
        let mut game_map = GameMap::new();

        world.register::<components::Position>();
        world.register::<components::Moved>();
        world.register::<components::material::Material>();

        game_map.mark_tile(&target, TileProperties::BLOCKED);

        world.insert(game_map);

        let ent_player = world
            .create_entity()
            .with(start.clone())
            .with(components::Moved {
                from: start.clone(),
                to: target.clone(),
            })
            .build();

        let mut collisions_solid = CollisionsSolid;
        collisions_solid.run_now(&world);
        world.maintain();

        let read_moved = world.read_storage::<components::Moved>();
        let player_move = read_moved.get(ent_player);

        match player_move {
            None => {
                panic!("Should still have a Moved component");
            }
            Some(_) => (),
        };
    }

    #[test]
    fn doesnt_affect_nonsolid_targets() {
        let mut world = World::new();
        let start = components::Position::new(3, -4);
        let target = components::Position::new(3, -5);
        let game_map = GameMap::new();

        world.register::<components::Position>();
        world.register::<components::Moved>();
        world.register::<components::material::Material>();

        world.insert(game_map);

        let ent_player = world
            .create_entity()
            .with(start.clone())
            .with(components::Moved {
                from: start.clone(),
                to: target.clone(),
            })
            .build();

        let mut collisions_solid = CollisionsSolid;
        collisions_solid.run_now(&world);
        world.maintain();

        let read_moved = world.read_storage::<components::Moved>();
        let player_move = read_moved.get(ent_player);

        match player_move {
            None => {
                panic!("Should still have a Moved component");
            }
            Some(_) => (),
        };
    }

    #[test]
    fn cancels_move_through_solid_tile() {
        let mut world = World::new();
        let start = components::Position::new(3, -4);
        let target = components::Position::new(3, -5);
        let mut game_map = GameMap::new();

        world.register::<components::Position>();
        world.register::<components::Moved>();
        world.register::<components::material::Material>();

        game_map.mark_tile(&target, TileProperties::BLOCKED);

        world.insert(game_map);

        let ent_player = world
            .create_entity()
            .with(start.clone())
            .with(components::Moved {
                from: start.clone(),
                to: target.clone(),
            })
            .with(components::material::flesh())
            .build();

        let mut collisions_solid = CollisionsSolid;
        collisions_solid.run_now(&world);
        world.maintain();

        let read_moved = world.read_storage::<components::Moved>();
        let player_move = read_moved.get(ent_player);

        match player_move {
            None => (),
            Some(_) => {
                panic!("Should not still have Moved component");
            }
        };
    }
}
