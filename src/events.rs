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

// fn my_recv_system(mut state: ResMut<MyState>, events: Res<Events<MyEvent>>) {
//     for ev in state.reader.iter(&events) {
//         // do something with `ev`
//         println!("event")
//     }
// }

// fn my_send_system(mut events: ResMut<Events<MyEvent>>) {
//     events.send(MyEvent);
// }
