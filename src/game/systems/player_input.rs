use super::super::{components, input, resources};
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct PlayerInput;

impl<'a> System<'a> for PlayerInput {
    type SystemData = (
        ReadStorage<'a, components::Player>,
        ReadStorage<'a, components::Position>,
        WriteStorage<'a, components::Moved>,
        Read<'a, resources::PendingAction>,
        specs::Entities<'a>,
        Read<'a, specs::LazyUpdate>,
    );

    fn run(
        &mut self,
        (player, position, mut moved, pending_action, entities, updater): Self::SystemData,
    ) {
        use specs::Join;

        let pending_action = &pending_action.0;

        match pending_action {
            Some(input::Action::Up) => {
                for (entity, pos, _, mv) in
                    (&entities, &position, &player, (&mut moved).maybe()).join()
                {
                    if let Some(mv) = mv {
                        mv.to = pos.up();
                    } else {
                        let mv = components::Moved {
                            from: pos.clone(),
                            to: pos.up(),
                        };

                        updater.insert(entity, mv);
                    }
                }
            }

            Some(input::Action::Down) => {
                for (entity, pos, _, mv) in
                    (&entities, &position, &player, (&mut moved).maybe()).join()
                {
                    if let Some(mv) = mv {
                        mv.to = pos.down();
                    } else {
                        let mv = components::Moved {
                            from: pos.clone(),
                            to: pos.down(),
                        };

                        updater.insert(entity, mv);
                    }
                }
            }

            Some(input::Action::Right) => {
                for (entity, pos, _, mv) in
                    (&entities, &position, &player, (&mut moved).maybe()).join()
                {
                    if let Some(mv) = mv {
                        mv.to = pos.right();
                    } else {
                        let mv = components::Moved {
                            from: pos.clone(),
                            to: pos.right(),
                        };

                        updater.insert(entity, mv);
                    }
                }
            }

            Some(input::Action::Left) => {
                for (entity, pos, _, mv) in
                    (&entities, &position, &player, (&mut moved).maybe()).join()
                {
                    if let Some(mv) = mv {
                        mv.to = pos.left();
                    } else {
                        let mv = components::Moved {
                            from: pos.clone(),
                            to: pos.left(),
                        };

                        updater.insert(entity, mv);
                    }
                }
            }

            Some(_) => (),

            None => (),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, RunNow, World, WorldExt};

    fn test_movement(
        pending_action: Option<input::Action>,
        start: components::Position,
        expected: components::Position,
    ) {
        let mut world = World::new();

        world.register::<components::Player>();
        world.register::<components::Position>();
        world.register::<components::Moved>();

        world.insert(resources::PendingAction(pending_action));

        let ent_player = world
            .create_entity()
            .with(start.clone())
            .with(components::Player)
            .build();

        let ent_npc = world.create_entity().with(start.clone()).build();

        let mut player_input = PlayerInput;
        player_input.run_now(&world);
        world.maintain();

        let read_moved = world.read_storage::<components::Moved>();
        let player_move = read_moved.get(ent_player);

        if start != expected {
            match player_move {
                None => panic!("Player Moved component not found at all"),
                Some(mv) => {
                    assert_eq!(mv.from, start);
                    assert_eq!(mv.to, expected);
                }
            };
        } else {
            match player_move {
                None => (),
                Some(_) => {
                    panic!(
                        "Did not expect to see a Moved component be added when expected == start"
                    );
                }
            }
        }

        // Make sure we don't move things without the Player component
        let npc_move = read_moved.get(ent_npc);

        match npc_move {
            None => (),
            Some(_) => {
                panic!("Should not have found a Moved component on NPC");
            }
        };
    }

    #[test]
    fn doesnt_move_when_no_actions_pending() {
        test_movement(
            None,
            components::Position::new(5, -3),
            components::Position::new(5, -3),
        );
    }

    #[test]
    fn moves_up_when_pressed() {
        // Negative Y is up
        test_movement(
            Some(input::Action::Up),
            components::Position::new(5, -3),
            components::Position::new(5, -4),
        );
    }

    #[test]
    fn moves_down_when_pressed() {
        // Positive Y is down
        test_movement(
            Some(input::Action::Down),
            components::Position::new(5, -3),
            components::Position::new(5, -2),
        );
    }

    #[test]
    fn moves_right_when_pressed() {
        test_movement(
            Some(input::Action::Right),
            components::Position::new(5, -3),
            components::Position::new(6, -3),
        );
    }

    #[test]
    fn moves_left_when_pressed() {
        test_movement(
            Some(input::Action::Left),
            components::Position::new(5, -3),
            components::Position::new(4, -3),
        );
    }
}
