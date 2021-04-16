use crate::camera_effect::*;
use bevy::prelude::*;

#[derive(Debug)]
pub struct MyEvent(i32, i32);

impl MyEvent {
    pub fn new(x: i32, y: i32) -> Self {
        MyEvent(x, y)
    }
}

/// 事件监听
pub fn event_listener_system(
    time: Res<Time>,
    mut camera_data: ResMut<CameraData>,
    mut state: EventReader<MyEvent>,
) {
    let _delta_seconds = f32::min(0.2, time.delta_seconds());

    for ev in state.iter() {
        // do something with `ev`
        println!("my event, {:?}", *ev);
        camera_data.state = CameraState::Shake;
    }
}
