use super::super::{components, input, resources};
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct PlayerMovement;

impl<'a> System<'a> for PlayerMovement {
    type SystemData = (
        ReadStorage<'a, components::Player>,
        WriteStorage<'a, components::Position>,
        Read<'a, resources::PendingAction>,
        Read<'a, resources::entity_grid::EntityGrid>,
    );

    fn run(&mut self, (player, mut position, pending_action, entity_grid): Self::SystemData) {
        use resources::entity_grid::{Coordinate, EntityGrid};
        use specs::Join;

        let entity_grid = &entity_grid;
        let pending_action = &pending_action.0;

        match pending_action {
            Some(input::Action::Up) => {
                for (pos, _) in (&mut position, &player).join() {
                    // Negative Y is up
                    pos.y -= 1.;
                }
            }
            Some(input::Action::Down) => {
                for (pos, _) in (&mut position, &player).join() {
                    // Positive Y is down
                    pos.y += 1.;
                }
            }
            Some(input::Action::Right) => {
                for (pos, _) in (&mut position, &player).join() {
                    pos.x += 1.;
                }
            }
            Some(input::Action::Left) => {
                for (pos, _) in (&mut position, &player).join() {
                    pos.x -= 1.;
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
        start_x: i32,
        start_y: i32,
        expected_x: i32,
        expected_y: i32,
    ) {
        let mut world = World::new();

        world.register::<components::Player>();
        world.register::<components::Position>();

        world.insert(resources::PendingAction(pending_action));
        world.insert(resources::entity_grid::EntityGrid::new());

        let ent_player = world
            .create_entity()
            .with(components::Position {
                x: start_x as f64,
                y: start_y as f64,
            })
            .with(components::Player)
            .build();

        let ent_npc = world
            .create_entity()
            .with(components::Position {
                x: start_x as f64,
                y: start_y as f64,
            })
            .build();

        let mut player_movement = PlayerMovement;
        player_movement.run_now(&world);
        world.maintain();

        let read_pos = world.read_storage::<components::Position>();
        let player_pos = read_pos.get(ent_player);

        match player_pos {
            None => panic!("Not found at all"),
            Some(pos) => {
                assert_eq!(pos.x as i32, expected_x as i32);
                assert_eq!(pos.y as i32, expected_y as i32);
            }
        };

        // Make sure we don't move things without the Player component
        let npc_pos = read_pos.get(ent_npc);

        match npc_pos {
            None => panic!("Not found at all"),
            Some(pos) => {
                assert_eq!(pos.x as i32, start_x as i32);
                assert_eq!(pos.y as i32, start_y as i32);
            }
        };
    }

    #[test]
    fn doesnt_move_when_no_actions_pending() {
        test(None, 5, -3, 5, -3);
    }

    #[test]
    fn moves_up_when_pressed() {
        // Negative Y is up
        test(Some(input::Action::Up), 5, -3, 5, -4);
    }

    #[test]
    fn moves_down_when_pressed() {
        // Positive Y is down
        test(Some(input::Action::Down), 5, -3, 5, -2);
    }

    #[test]
    fn moves_right_when_pressed() {
        test(Some(input::Action::Right), 5, -3, 6, -3);
    }

    #[test]
    fn moves_left_when_pressed() {
        test(Some(input::Action::Left), 5, -3, 4, -3);
    }
}
