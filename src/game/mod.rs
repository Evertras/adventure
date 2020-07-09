pub mod components;
pub mod entities;
pub mod input;
pub mod resources;
pub mod systems;

use input::Action;

use systems::{
    collisions_solid::CollisionsSolid,
    movement_apply::MovementApply,
    player_input::PlayerInput,
    render::{Render, Renderer},
    sync_game_map::SyncGameMap,
    visibility::Visibility,
};

use specs::{DispatcherBuilder, World, WorldExt};

pub fn run<T: Renderer, U: input::Buffer>(renderer: T, mut input: U) {
    let mut world = World::new();

    world.insert(resources::DeltaTime(std::time::Duration::from_secs(1)));
    world.insert(resources::CameraCenter { x: 0, y: 0 });
    world.insert(resources::PendingAction(None));

    let render = Render::new(renderer);

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerInput, "player_input", &[])
        .with(SyncGameMap, "sync_game_map", &[])
        .with(
            CollisionsSolid,
            "collisions_solid",
            &["sync_game_map", "player_input"],
        )
        .with(
            MovementApply,
            "movement_apply",
            &["player_input", "collisions_solid"],
        )
        .with(Visibility, "visibility", &["movement_apply"])
        .with_thread_local(render)
        .build();

    dispatcher.setup(&mut world);

    entities::player::create_in(&mut world, components::Position::new(0, 0));
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
