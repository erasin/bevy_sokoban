mod loading;
mod menu;

mod actions;
pub mod camera_effect;
pub mod components;
pub mod data;
pub mod debug;
pub mod events;
pub mod grid;
pub mod map;
pub mod state;
pub mod systems;
pub mod ui;

use bevy::prelude::*;

use actions::ActionsPlugin;
use camera_effect::*;
use data::*;
use debug::DebugPlugin;
use events::*;
use grid::GridPlugin;
use map::{Map, MapPlugin};
use state::*;
use systems::*;
use ui::*;

use camera_effect::CameraEffectPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;

/// 瓦片宽度大小设定
const TILED_WIDTH: f32 = 32.0;
/// 瓦片放大比例
const SCALE: f32 = 2.0;

pub struct Plugins;

impl Plugin for Plugins {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            // .add_plugin(GridPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(CameraEffectPlugin::new(0.5))
            .add_plugin(UIPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayPlugin)
            .init_resource::<GameData>()
            .add_plugin(DebugPlugin);
    }
}
