use super::super::{components, input, resources};
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct PlayerMovement;

impl<'a> System<'a> for PlayerMovement {
    type SystemData = (
        ReadStorage<'a, components::Player>,
        WriteStorage<'a, components::Position>,
        Read<'a, resources::PendingAction>,
    );

    fn run(&mut self, (player, mut position, pending_action): Self::SystemData) {
        use specs::Join;

        let pending_action = &pending_action.0;

        match pending_action {
            Some(input::Action::Up) => {
                for (pos, _) in (&mut position, &player).join() {
                    pos.move_up();
                }
            }
            Some(input::Action::Down) => {
                for (pos, _) in (&mut position, &player).join() {
                    pos.move_down();
                }
            }
            Some(input::Action::Right) => {
                for (pos, _) in (&mut position, &player).join() {
                    pos.move_right();
                }
            }
            Some(input::Action::Left) => {
                for (pos, _) in (&mut position, &player).join() {
                    pos.move_left();
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

    fn test(
        pending_action: Option<input::Action>,
        start: components::Position,
        expected: components::Position,
    ) {
        let mut world = World::new();

        world.register::<components::Player>();
        world.register::<components::Position>();

        world.insert(resources::PendingAction(pending_action));

        let ent_player = world
            .create_entity()
            .with(start.clone())
            .with(components::Player)
            .build();

        let ent_npc = world.create_entity().with(start.clone()).build();

        let mut player_movement = PlayerMovement;
        player_movement.run_now(&world);
        world.maintain();

        let read_pos = world.read_storage::<components::Position>();
        let player_pos = read_pos.get(ent_player);

        match player_pos {
            None => panic!("Not found at all"),
            Some(pos) => {
                assert_eq!(pos.x as i32, expected.x as i32);
                assert_eq!(pos.y as i32, expected.y as i32);
            }
        };

        // Make sure we don't move things without the Player component
        let npc_pos = read_pos.get(ent_npc);

        match npc_pos {
            None => panic!("Not found at all"),
            Some(pos) => {
                assert_eq!(pos.x as i32, start.x as i32);
                assert_eq!(pos.y as i32, start.y as i32);
            }
        };
    }

    #[test]
    fn doesnt_move_when_no_actions_pending() {
        test(
            None,
            components::Position::new(5, -3),
            components::Position::new(5, -3),
        );
    }

    #[test]
    fn moves_up_when_pressed() {
        // Negative Y is up
        test(
            Some(input::Action::Up),
            components::Position::new(5, -3),
            components::Position::new(5, -4),
        );
    }

    #[test]
    fn moves_down_when_pressed() {
        // Positive Y is down
        test(
            Some(input::Action::Down),
            components::Position::new(5, -3),
            components::Position::new(5, -2),
        );
    }

    #[test]
    fn moves_right_when_pressed() {
        test(
            Some(input::Action::Right),
            components::Position::new(5, -3),
            components::Position::new(6, -3),
        );
    }

    #[test]
    fn moves_left_when_pressed() {
        test(
            Some(input::Action::Left),
            components::Position::new(5, -3),
            components::Position::new(4, -3),
        );
    }
}
