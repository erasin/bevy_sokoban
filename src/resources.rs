use bevy::prelude::*;

pub struct ResourceData {
    pub texture_atlas_sheet: Handle<TextureAtlas>,
    pub texture_atlas_player: Handle<TextureAtlas>,
    pub texture_atlas_box_blue: Handle<TextureAtlas>,
    pub ui_font: Handle<Font>,
    pub music_wall: Handle<AudioSource>,
}

#[derive(Default)]
pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage_before(
            bevy::app::startup_stage::STARTUP,
            "INITRES",
            SystemStage::serial(),
        );
        app.add_startup_system_to_stage("INITRES", setup_system.system());
    }
}

fn setup_system(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    _textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("setup res");
    // sheet
    let texture_handle = asset_server.load("textures/sheet.png");
    // let texture = textures.get(texture_handle.clone_weak());
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 5, 1);
    let texture_atlas_sheet = texture_atlases.add(texture_atlas);
    // 0 player 1 floor 2 spot 3 wall 4 box

    // player
    let texture_handle = asset_server.load("textures/player_sheet.png");
    // let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 1);
    let texture_atlas_player = texture_atlases.add(texture_atlas);

    // blue box
    let texture_handle = asset_server.load("textures/box_blue_sheet.png");
    // let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 1);
    let texture_atlas_box_blue = texture_atlases.add(texture_atlas);

    // 字体
    let ui_font = asset_server.load("fonts/KenneyFuture.ttf");

    let music_wall = asset_server.load("sounds/wall.wav");

    let resource = ResourceData {
        texture_atlas_sheet,
        texture_atlas_player,
        texture_atlas_box_blue,
        ui_font,
        music_wall,
    };

    commands.insert_resource(resource);
}
