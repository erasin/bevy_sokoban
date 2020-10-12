// 面板

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Default)]
pub struct DebugPlugin;

pub struct FpsText;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_system.system())
            .add_system(fps_text_update_system.system());
    }
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("load debug");

    let font_handle = asset_server.load("assets/fonts/KenneyFuture.ttf").unwrap();

    commands
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                },
            },
            ..Default::default()
        })
        .with(FpsText);
}

fn fps_text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<(&mut Text, &FpsText)>) {
    for (mut text, _tag) in &mut query.iter() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}
