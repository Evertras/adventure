use super::super::{components, resources};
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        Read<'a, resources::DeltaTime>,
        WriteStorage<'a, components::Position>,
        ReadStorage<'a, components::Velocity>,
    );

    fn run(&mut self, (delta, mut position, vel): Self::SystemData) {
        use specs::Join;

        let delta = delta.0.as_secs_f64();

        for (pos, vel) in (&mut position, &vel).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, RunNow, World, WorldExt};

    #[test]
    fn moves() {
        let mut world = World::new();
        let start_x = 3.0;
        let start_y = 5.5;
        let vel_x = 130.0;
        let vel_y = -33.3;
        let delta_secs: f64 = 0.5;
        let delta = resources::DeltaTime(std::time::Duration::from_secs_f64(delta_secs));

        world.register::<components::Position>();
        world.register::<components::Velocity>();

        world.insert(delta);

        let ent = world
            .create_entity()
            .with(components::Position {
                x: start_x,
                y: start_y,
            })
            .with(components::Velocity { x: vel_x, y: vel_y })
            .build();

        let mut movement = Movement;
        movement.run_now(&world);
        world.maintain();

        let read_pos = world.read_storage::<components::Position>();

        let found_pos = read_pos.get(ent);

        match found_pos {
            None => panic!("Not found at all"),
            Some(pos) => {
                let expected_x = (start_x + vel_x * delta_secs) as i32;
                let expected_y = (start_y + vel_y * delta_secs) as i32;

                assert_ne!(expected_x, start_x as i32);
                assert_ne!(expected_y, start_y as i32);

                assert_eq!(pos.x as i32, expected_x);
                assert_eq!(pos.y as i32, expected_y);
            }
        };
    }
}
