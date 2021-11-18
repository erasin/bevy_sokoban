use crate::全局状态;

use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct 加载素材库插件;

impl Plugin for 加载素材库插件 {
    fn build(&self, app: &mut AppBuilder) {
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
