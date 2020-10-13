use crate::camera_effect::*;
use bevy::prelude::*;

#[derive(Debug)]
pub struct MyEvent(i32, i32);

impl MyEvent {
    pub fn new(x: i32, y: i32) -> Self {
        MyEvent(x, y)
    }
}

#[derive(Default)]
pub struct MyEventListenerState {
    pub reader: EventReader<MyEvent>,
}

/// 事件监听
pub fn event_listener_system(
    time: Res<Time>,
    mut state: ResMut<MyEventListenerState>,
    mut camera_data: ResMut<CameraData>,
    events: Res<Events<MyEvent>>,
) {
    let _delta_seconds = f32::min(0.2, time.delta_seconds);

    for ev in state.reader.iter(&events) {
        // do something with `ev`
        println!("my event, {:?}", ev);
        camera_data.state = CameraState::Shake;
    }
}
