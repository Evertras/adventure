pub mod entity_grid;

#[derive(Default)]
pub struct DeltaTime(pub std::time::Duration);

#[derive(Default)]
pub struct CameraCenter {
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct PendingAction(pub Option<super::input::Action>);
