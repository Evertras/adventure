use super::super::{components, resources};
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct Visibility;

impl<'a> System<'a> for Visibility {
    type SystemData = (
        specs::Entities<'a>,
        Option<Read<'a, resources::Player>>,
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::material::Material>,
        ReadStorage<'a, components::Shape>,
        WriteStorage<'a, components::Visible>,
    );

    fn run(
        &mut self,
        (entities, player, positions, materials, shapes, mut visibles): Self::SystemData,
    ) {
        use specs::Join;

        if let Some(player) = player {
            visibles.clear();

            let ent_player = player.ent;
            let pos_player = positions.get(ent_player).unwrap();
            let mut max_vision_blocked = std::collections::HashMap::new();

            let get_bucket = |theta: f32| -> i32 {
                (theta * 10.) as i32
            };

            // This isn't great but it's simple and works for now
            for (pos, material, shape) in
                (&positions, &materials, &shapes).join()
            {
                if components::Shape::FullBlock != *shape || !material.opaque {
                    continue;
                }

                let theta = pos_player.theta(pos);
                let distance = pos_player.distance_squared(pos);

                let existing_max = max_vision_blocked.entry(get_bucket(theta)).or_insert(100000_f32);

                if *existing_max > distance {
                    *existing_max = distance;
                }
            }

            for (entity, pos, material) in
                (&entities, &positions, &materials).join()
            {
                if !material.visible {
                    continue;
                }

                let theta = pos_player.theta(&pos);
                let distance = pos_player.distance_squared(&pos);

                if let Some(max_distance) = max_vision_blocked.get(&get_bucket(theta)) {
                    if distance <= *max_distance {
                        visibles.insert(entity, components::Visible).unwrap();
                    }
                } else {
                    visibles.insert(entity, components::Visible).unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::entities;
    use specs::{Builder, RunNow, World, WorldExt};

    fn build_world() -> specs::World {
        let mut world = World::new();

        world.register::<components::Position>();
        world.register::<components::Player>();
        world.register::<components::material::Material>();
        world.register::<components::Shape>();
        world.register::<components::Visible>();
        world.register::<components::Sprite>();

        world
    }

    fn add_stone_wall(world: &mut specs::World, pos: components::Position) -> specs::Entity {
        world
            .create_entity()
            .with(pos.clone())
            .with(components::material::stone())
            .with(components::Shape::FullBlock)
            .build()
    }

    fn add_generic_medium_creature(world: &mut specs::World, pos: components::Position) -> specs::Entity {
        world
            .create_entity()
            .with(pos.clone())
            .with(components::material::flesh())
            .with(components::Shape::Medium)
            .build()
    }

    #[test]
    fn marks_player_visible() {
        let mut world = build_world();

        let ent_player = entities::player::create_in(&mut world, components::Position::new(0, 0));

        let mut visibility = Visibility;
        visibility.run_now(&world);
        world.maintain();

        let read_visible = world.read_storage::<components::Visible>();
        let player_visible = read_visible.get(ent_player);

        match player_visible {
            None => panic!("Player not visible but should be"),
            Some(_) => (),
        };
    }

    #[test]
    fn marks_opaque_entities_visible() {
        let mut world = build_world();
        let pos_player = components::Position::new(3, -4);
        let pos_other = pos_player.right();

        let ent_player = entities::player::create_in(&mut world, pos_player.clone());
        let ent_other = add_generic_medium_creature(&mut world, pos_other.clone());

        let mut visibility = Visibility;
        visibility.run_now(&world);
        world.maintain();

        let read_visible = world.read_storage::<components::Visible>();
        let other_visible = read_visible.get(ent_other);

        match other_visible {
            None => panic!("Other entity not visible but should be"),
            Some(_) => (),
        };
    }

    #[test]
    fn does_not_mark_creature_visible_if_behind_wall() {
        let mut world = build_world();
        let pos_player = components::Position::new(3, -4);
        let pos_wall = pos_player.right();
        let pos_creature = pos_wall.right();

        let ent_player = entities::player::create_in(&mut world, pos_player);
        let ent_wall = add_stone_wall(&mut world, pos_wall);
        let ent_creature = add_generic_medium_creature(&mut world, pos_creature);

        let mut visibility = Visibility;
        visibility.run_now(&world);
        world.maintain();

        let read_visible = world.read_storage::<components::Visible>();
        let wall_visible = read_visible.get(ent_wall);
        let creature_visible = read_visible.get(ent_creature);

        match wall_visible {
            None => panic!("Wall not visible but should be"),
            Some(_) => (),
        };

        match creature_visible {
            None => (),
            Some(_) => panic!("Creature visible but should not be"),
        };
    }
}
