use bevy::{prelude::*, render::pass::ClearColor};
use sokoban;

fn main() {
    dotenv::dotenv().ok();
    let mut app = App::build();

    app.insert_resource(ClearColor(Color::hex("E0E0E0").unwrap()))
        .insert_resource(WindowDescriptor {
            title: "sokoban!".to_string(),
            width: 800.0,
            height: 800.0,
            vsync: true,
            // resizable: false,
            // mode: WindowMode::Fullscreen { use_size: false },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(sokoban::组件集合);
    app.run();
}
