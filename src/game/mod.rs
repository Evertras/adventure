pub mod components;
pub mod entities;
pub mod input;
pub mod resources;
pub mod systems;

use input::Action;
use systems::render::Renderer;

use systems::movement_apply::MovementApply;
use systems::player_input::PlayerInput;

use specs::{DispatcherBuilder, World, WorldExt};

pub fn run<T: Renderer, U: input::Buffer>(renderer: T, mut input: U) {
    let mut world = World::new();

    world.insert(resources::DeltaTime(std::time::Duration::from_secs(1)));
    world.insert(resources::CameraCenter { x: 0, y: 0 });
    world.insert(resources::PendingAction(None));

    let render = systems::render::Render::new(renderer);

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerInput, "player_input", &[])
        .with(MovementApply, "movement_apply", &["player_input"])
        .with_thread_local(render)
        .build();

    dispatcher.setup(&mut world);

    entities::player::create_in(&mut world);
    entities::map::create_in(&mut world);

    loop {
        {
            let mut pending_actions = input.step();

            for action in &pending_actions {
                match action {
                    // Just nope out
                    Action::HardExit => return,
                    _ => (),
                }
            }

            let mut action_writer = world.write_resource::<resources::PendingAction>();

            *action_writer = resources::PendingAction(pending_actions.pop());
        }

        dispatcher.dispatch(&mut world);
        world.maintain();

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
