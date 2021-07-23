use crate::全局状态;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct AssetPaths {
    pub font_ui: &'static str,
    pub audio_wall: &'static str,
    pub texture_sheet: &'static str,
    pub texture_player: &'static str,
    pub texture_box_blue: &'static str,
}

#[allow(dead_code)]
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
        // app.add_system_set(SystemSet::on_enter(全局状态::加载中).with_system(开始加载.system()))
        // .add_system_set(SystemSet::on_update(全局状态::加载中).with_system(检查变动.system()));

        AssetLoader::new(全局状态::加载中, 全局状态::菜单)
            .with_collection::<字体素材>()
            .with_collection::<音频素材>()
            .with_collection::<纹理素材图片>()
            .init_resource::<纹理素材>()
            .build(app);
    }
}

pub struct LoadingState {
    材质: Vec<HandleUntyped>,
    字体: Vec<HandleUntyped>,
    音频: Vec<HandleUntyped>,
}

#[derive(AssetCollection)]
pub struct 字体素材 {
    #[asset(path = "fonts/KenneyFuture.ttf")]
    pub font_ui: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct 音频素材 {
    #[asset(path = "sounds/wall.wav")]
    pub audio_wall: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct 纹理素材图片 {
    #[asset(path = "textures/sheet.png")]
    pub 纹理表: Handle<Texture>,
    #[asset(path = "textures/player_sheet.png")]
    pub 用户: Handle<Texture>,
    #[asset(path = "textures/box_blue_sheet.png")]
    pub 蓝箱子: Handle<Texture>,
}

pub struct 纹理素材 {
    pub 纹理表: Handle<TextureAtlas>,
    pub 用户: Handle<TextureAtlas>,
    pub 蓝箱子: Handle<TextureAtlas>,
}

impl FromWorld for 纹理素材 {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let assets = cell
            .get_resource::<纹理素材图片>()
            .expect("SpriteSheet not loaded");
        let mut atlases = cell
            .get_resource_mut::<Assets<TextureAtlas>>()
            .expect("TextureAtlases missing");
        纹理素材 {
            纹理表: atlases.add(TextureAtlas::from_grid(
                assets.纹理表.clone(),
                Vec2::new(32., 32.),
                5,
                1,
            )),
            用户: atlases.add(TextureAtlas::from_grid(
                assets.用户.clone(),
                Vec2::new(32., 32.),
                3,
                1,
            )),
            蓝箱子: atlases.add(TextureAtlas::from_grid(
                assets.蓝箱子.clone(),
                Vec2::new(32., 32.),
                2,
                1,
            )),
        }
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

    commands.insert_resource(字体素材 {
        font_ui: asset_server.get_handle(PATHS.font_ui),
    });

    commands.insert_resource(音频素材 {
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

    commands.insert_resource(纹理素材 {
        纹理表: texture_atlas_sheet,
        用户: texture_atlas_player,
        蓝箱子: texture_atlas_box_blue,
    });

    state.set(全局状态::菜单).unwrap();
}
