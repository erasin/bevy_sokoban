mod paths;

use crate::loading::paths::PATHS;
use crate::GameState;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_state.system()));
    }
}

pub struct LoadingState {
    textures: Vec<HandleUntyped>,
    fonts: Vec<HandleUntyped>,
    audio: Vec<HandleUntyped>,
}

pub struct FontAssets {
    pub font_ui: Handle<Font>,
}

pub struct AudioAssets {
    pub audio_wall: Handle<AudioSource>,
}

pub struct TextureAssets {
    pub texture_sheet: Handle<TextureAtlas>,
    pub texture_player: Handle<TextureAtlas>,
    pub texture_box_blue: Handle<TextureAtlas>,
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.font_ui));

    let mut audio: Vec<HandleUntyped> = vec![];
    audio.push(asset_server.load_untyped(PATHS.audio_wall));

    let mut textures: Vec<HandleUntyped> = vec![];
    textures.push(asset_server.load_untyped(PATHS.texture_sheet));
    textures.push(asset_server.load_untyped(PATHS.texture_player));
    textures.push(asset_server.load_untyped(PATHS.texture_box_blue));

    commands.insert_resource(LoadingState {
        textures,
        fonts,
        audio,
    });
}

fn check_state(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.fonts.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.textures.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.audio.iter().map(|handle| handle.id))
    {
        return;
    }

    commands.insert_resource(FontAssets {
        font_ui: asset_server.get_handle(PATHS.font_ui),
    });

    commands.insert_resource(AudioAssets {
        audio_wall: asset_server.get_handle(PATHS.audio_wall),
    });

    // sheet
    let texture_handle = asset_server.get_handle(PATHS.texture_sheet);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 5, 1);
    let texture_atlas_sheet = texture_atlases.add(texture_atlas);
    // 0 player 1 floor 2 spot 3 wall 4 box

    // player
    let texture_handle = asset_server.get_handle(PATHS.texture_player);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 1);
    let texture_atlas_player = texture_atlases.add(texture_atlas);

    // blue box
    let texture_handle = asset_server.get_handle(PATHS.texture_box_blue);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 1);
    let texture_atlas_box_blue = texture_atlases.add(texture_atlas);

    commands.insert_resource(TextureAssets {
        texture_sheet: texture_atlas_sheet,
        texture_player: texture_atlas_player,
        texture_box_blue: texture_atlas_box_blue,
    });

    state.set(GameState::Menu).unwrap();
}
