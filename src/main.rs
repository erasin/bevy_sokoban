mod camera_effect;
mod components;
mod data;
mod debug;
mod events;
mod grid;
mod map;
mod resources;
mod state;
mod systems;
mod ui;

use bevy::{prelude::*, render::pass::ClearColor};
use camera_effect::*;
use data::*;
use debug::DebugPlugin;
use events::*;
use grid::GridPlugin;
use map::MapPlugin;
use resources::*;
use systems::*;
use ui::*;

/// 瓦片宽度大小设定
const TILED_WIDTH: f32 = 32.0;
/// 瓦片放大比例
const SCALE: f32 = 2.0;

fn main() {
    dotenv::dotenv().ok();
    App::build()
        .add_resource(ClearColor(Color::hex("E0E0E0").unwrap()))
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
        .init_resource::<GameData>()
        .init_resource::<MyEventListenerState>()
        .add_event::<MyEvent>()
        .add_plugin(ResourcePlugin::default())
        .add_plugin(CameraEffectPlugin::new(0.5))
        .add_plugin(UIPlugin::default())
        .add_plugin(DebugPlugin::default())
        .add_plugin(GridPlugin::default())
        .add_plugin(MapPlugin::default())
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .add_system_to_stage(stage::EVENT_UPDATE, player_movement_system.system())
        .add_system(box_spot_system.system())
        .add_system(position_system.system())
        .add_system(scoreboard_system.system())
        .add_system(event_listener_system.system())
        .run();
}

/// 初始化处理
pub fn setup(mut commands: Commands) {
    println!("setup main");
    // commands
    //     .spawn(Camera2dComponents::default())
    //     .with(CameraTarget) // 加载相机
    //     .spawn(UiCameraComponents::default()); // 加载ui层
}
