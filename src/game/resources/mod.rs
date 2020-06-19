#[derive(Default)]
pub struct DeltaTime(pub std::time::Duration);

#[derive(Default)]
pub struct CameraCenter {
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct PendingActions {
    pub actions: Vec<super::input::Action>,
}
