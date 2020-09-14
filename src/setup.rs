use crate::components::*;
use crate::resources::*;
use crate::TILED_WIDTH;
use bevy::prelude::*;
use std::env;

/// 初始化处理
/// 地区加载处理
pub fn setup(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
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
    let ui_font = asset_server.load("assets/fonts/KenneyFuture.ttf").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // .spawn(SpriteComponents {
        //     material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        //     translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
        //     sprite: Sprite {
        //         size: Vec2::new(50.0, 50.0),
        //         resize_mode: SpriteResizeMode::Manual,
        //     },
        //     ..Default::default()
        // })
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal,
            ..Default::default()
        })
        .with(UIBTN)
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: "Button".to_string(),
                    font: ui_font,
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                },
                ..Default::default()
            });
        });
    // .spawn(NodeComponents {
    //     style: Style {
    //         size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    //         justify_content: JustifyContent::Center,
    //         align_items: AlignItems::Center,
    //         ..Default::default()
    //     },
    //     material: materials.add(Color::rgba(0.04, 0.04, 0.04, 0.0).into()),
    //     ..Default::default()
    // })
    // .with_children(|parent| {
    //     parent
    // .spawn(NodeComponents {
    //     style: Style {
    //         size: Size::new(Val::Px(80.0), Val::Px(80.0)),
    //         ..Default::default()
    //     },
    //     material: materials.add(Color::rgb(0.08, 0.08, 1.0).into()),
    //     ..Default::default()
    // })

    // .spawn(TextComponents {
    //     style: Style {
    //         size: Size::new(Val::Px(80.0), Val::Px(80.0)),
    //         ..Default::default()
    //     },
    //     text: Text {
    //         value: "FPS:".to_string(),
    //         font: ui_font,
    //         style: TextStyle {
    //             font_size: 60.0,
    //             color: Color::WHITE,
    //         },
    //     },
    //     ..Default::default()
    // })
    // .with(UIFPS);
    // });

    // @bug: TextComponents && SpriteComponents

    // 地图加载
    // let map_string: &str = std::include_str!("../assets/m3.txt");
    let map_file = env::var("MAP").unwrap_or("./assets/m1.txt".to_string());
    // todo map
    *map = Map::load(map_file).unwrap();
    map.set_atlas(
        texture_atlas_sheet,
        texture_atlas_player,
        texture_atlas_box_blue,
    );
    map.render(&mut commands);

    let row_count = map.height as i32;
    let col_count = map.width as i32;

    let line_color = materials.add(Color::hex("000000").unwrap().into());
    let bounds = Vec2::new(400.0, 500.0);
    let def_width = TILED_WIDTH;

    for i in 0..=row_count {
        let j = i as f32;
        commands.spawn(SpriteComponents {
            material: line_color,
            translation: Translation(Vec3::new(def_width * j, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(2.0, bounds.y()),
                resize_mode: SpriteResizeMode::Automatic,
            },
            ..Default::default()
        });
    }

    for i in 0..col_count {
        let j = i as f32;
        commands.spawn(SpriteComponents {
            material: line_color,
            translation: Translation(Vec3::new(0.0, def_width * j, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x(), 2.0),
                resize_mode: SpriteResizeMode::Automatic,
            },
            ..Default::default()
        });
    }
}
