mod comments;
mod setup;
mod systems;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::pass::ClearColor};
use setup::setup;
use systems::*;

/// 瓦片宽度大小设定
const TILED_WIDTH: f32 = 32.0;
/// 瓦片放大比例
const SCALE: f32 = 2.0;

fn main() {
    dotenv::dotenv().ok();
    App::build()
        .add_resource(WindowDescriptor {
            title: "sokoban!".to_string(),
            width: 800,
            height: 800,
            vsync: true,
            // resizable: false,
            // mode: WindowMode::Fullscreen { use_size: false },
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_startup_system(setup.system())
        .add_system_to_stage(stage::FIRST, camera_system.system())
        .add_system(animate_sprite_system.system())
        .add_system_to_stage(stage::EVENT_UPDATE, player_movement_system.system())
        .add_system(box_spot_system.system())
        .add_system(position_system.system())
        .add_system(scoreboard_system.system())
        .add_system(event_listener_system.system())
        .add_system(fps_update_system.system())
        .run();
}
