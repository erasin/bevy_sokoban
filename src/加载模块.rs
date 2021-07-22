use crate::全局状态;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct AssetPaths {
    pub font_ui: &'static str,
    pub audio_wall: &'static str,
    pub texture_sheet: &'static str,
    pub texture_player: &'static str,
    pub texture_box_blue: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    font_ui: "fonts/KenneyFuture.ttf",
    audio_wall: "sounds/wall.wav",
    texture_sheet: "textures/sheet.png",
    texture_player: "textures/player_sheet.png",
    texture_box_blue: "textures/box_blue_sheet.png",
};

pub struct 加载素材库插件;

impl Plugin for 加载素材库插件 {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(全局状态::加载中).with_system(开始加载.system()))
            .add_system_set(SystemSet::on_update(全局状态::加载中).with_system(检查变动.system()));
    }
}

pub struct LoadingState {
    材质: Vec<HandleUntyped>,
    字体: Vec<HandleUntyped>,
    音频: Vec<HandleUntyped>,
}

pub struct FontAssets {
    pub font_ui: Handle<Font>,
}

pub struct AudioAssets {
    pub audio_wall: Handle<AudioSource>,
}

pub struct TextureAssets {
    pub 纹理表: Handle<TextureAtlas>,
    pub 用户: Handle<TextureAtlas>,
    pub 蓝箱子: Handle<TextureAtlas>,
}

fn 开始加载(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.font_ui));

    let mut audio: Vec<HandleUntyped> = vec![];
    audio.push(asset_server.load_untyped(PATHS.audio_wall));

    let mut textures: Vec<HandleUntyped> = vec![];
    textures.push(asset_server.load_untyped(PATHS.texture_sheet));
    textures.push(asset_server.load_untyped(PATHS.texture_player));
    textures.push(asset_server.load_untyped(PATHS.texture_box_blue));

    commands.insert_resource(LoadingState {
        材质: textures,
        字体: fonts,
        音频: audio,
    });
}

fn 检查变动(
    mut commands: Commands,
    mut state: ResMut<State<全局状态>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.字体.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.材质.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.音频.iter().map(|handle| handle.id))
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
        纹理表: texture_atlas_sheet,
        用户: texture_atlas_player,
        蓝箱子: texture_atlas_box_blue,
    });

    state.set(全局状态::菜单).unwrap();
}
