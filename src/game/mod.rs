pub mod components;
pub mod input;
pub mod resources;
pub mod systems;

use input::Action;
use systems::render::Renderer;

use specs::{Builder, DispatcherBuilder, World, WorldExt};

pub fn run<T: Renderer, U: input::Buffer>(renderer: T, mut input: U) {
    let mut world = World::new();

    world.register::<components::Position>();
    world.register::<components::Velocity>();
    world.register::<components::Draw>();

    world.insert(resources::DeltaTime(std::time::Duration::from_secs(1)));
    world.insert(resources::CameraCenter { x: 0, y: 0 });
    world.insert(resources::PendingActions { actions: vec![] });

    world
        .create_entity()
        .with(components::Position { x: 0., y: 0. })
        .with(components::Draw {
            fg_r: 255,
            fg_g: 128,
            fg_b: 128,
            bg_r: 0,
            bg_g: 128,
            bg_b: 128,
            rune: '@',
        })
        .build();

    let render = systems::render::Render::new(renderer);

    let mut dispatcher = DispatcherBuilder::new().with_thread_local(render).build();

    loop {
        {
            let pending_actions = input.step();

            for action in &pending_actions {
                match action {
                    // Just nope out
                    Action::HardExit => return,
                    _ => (),
                }
            }

            let mut action_writer = world.write_resource::<resources::PendingActions>();

            *action_writer = resources::PendingActions {
                actions: pending_actions,
            };
        }

        dispatcher.dispatch(&mut world);
        world.maintain();

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
