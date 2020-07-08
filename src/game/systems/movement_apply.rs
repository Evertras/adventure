use super::super::components;
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct MovementApply;

impl<'a> System<'a> for MovementApply {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        ReadStorage<'a, components::Moved>,
        specs::Entities<'a>,
        Read<'a, specs::LazyUpdate>,
    );

    fn run(&mut self, (mut position, moved, entities, updater): Self::SystemData) {
        use specs::Join;

        for (entity, pos, mv) in (&entities, &mut position, &moved).join() {
            pos.set(&mv.to);

            updater.remove::<components::Moved>(entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, RunNow, World, WorldExt};

    #[test]
    fn moves_to_pos() {
        let mut world = World::new();
        let start = components::Position::new(3, -4);
        let target = components::Position::new(3, -5);

        world.register::<components::Position>();
        world.register::<components::Moved>();

        let ent_player = world
            .create_entity()
            .with(start.clone())
            .with(components::Moved {
                from: start.clone(),
                to: target.clone(),
            })
            .build();

        let mut move_apply = MovementApply;
        move_apply.run_now(&world);
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
