use bevy::prelude::*;
use sokoban;

fn main() {
    dotenv::dotenv().ok();
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::hex("E0E0E0").unwrap()))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "sokoban!".to_string(),
                        width: 800.0,
                        height: 800.0,
                        // resizable: false,
                        // mode: WindowMode::Fullscreen { use_size: false },
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugins(sokoban::组件集合);
    app.run();
}
