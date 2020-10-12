use bevy::prelude::*;

pub struct ResourceLocal {
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
        app.add_startup_stage_before(bevy::app::startup_stage::STARTUP, "INITRES");
        app.add_startup_system_to_stage("INITRES", setup_system.system());
    }
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("setup res");
    // sheet
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/textures/sheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 5, 1);
    let texture_atlas_sheet = texture_atlases.add(texture_atlas);
    // 0 player 1 floor 2 spot 3 wall 4 box

    // player
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/textures/player_sheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 1);
    let texture_atlas_player = texture_atlases.add(texture_atlas);

    // blue box
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/textures/box_blue_sheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 1);
    let texture_atlas_box_blue = texture_atlases.add(texture_atlas);

    // 字体
    let ui_font = asset_server.load("assets/fonts/KenneyFuture.ttf").unwrap();

    let music_wall = asset_server.load("assets/sounds/wall.mp3").unwrap();

    let resource = ResourceLocal {
        texture_atlas_sheet,
        texture_atlas_player,
        texture_atlas_box_blue,
        ui_font,
        music_wall,
    };

    commands.insert_resource(resource);
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}
