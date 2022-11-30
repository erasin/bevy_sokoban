use crate::状态模块::全局状态;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct 加载素材库插件;

impl Plugin for 加载素材库插件 {
    fn build(&self, app: &mut App) {
        LoadingState::new(全局状态::加载中)
            .continue_to_state(全局状态::主菜单)
            .with_collection::<字体素材>()
            .with_collection::<音频素材>()
            // .with_collection::<纹理素材图片>()
            .with_collection::<纹理素材>()
            // .init_resource::<纹理素材>()
            .build(app);
    }
}

#[derive(AssetCollection, Resource)]
pub struct 字体素材 {
    #[asset(path = "fonts/KenneyFuture.ttf")]
    pub font_ui: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct 音频素材 {
    #[asset(path = "audio/wall.wav")]
    pub wall: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct 纹理素材 {
    #[asset(texture_atlas(
        tile_size_x = 32.,
        tile_size_y = 32.,
        columns = 5,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/sheet.png")]
    pub 纹理表: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 32.,
        tile_size_y = 32.,
        columns = 3,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/player_sheet.png")]
    pub 用户: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 32.,
        tile_size_y = 32.,
        columns = 2,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "textures/box_blue_sheet.png")]
    pub 蓝箱子: Handle<TextureAtlas>,
}
