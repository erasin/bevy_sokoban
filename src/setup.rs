use crate::comments::*;
use crate::{SCALE, TILED_WIDTH};
use bevy::prelude::*;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

/// 初始化处理
/// 地区加载处理
pub fn setup(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
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
    // let font_ui = asset_server.load("assets/fonts/KenneyFuture.ttf").unwrap();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(50.0, 50.0),
                resize_mode: SpriteResizeMode::Manual,
            },
            ..Default::default()
        });
    // .spawn(TextComponents {
    //     style: Style {
    //         align_self: AlignSelf::FlexEnd,
    //         ..Default::default()
    //     },
    //     text: Text {
    //         value: "FPS:".to_string(),
    //         font: font_ui,
    //         style: TextStyle {
    //             font_size: 60.0,
    //             color: Color::WHITE,
    //         },
    //     },
    //     ..Default::default()
    // });

    // @bug: TextComponents && SpriteComponents

    // 地图加载
    // let map_string: &str = std::include_str!("../assets/m3.txt");
    let map_file = env::var("MAP").unwrap_or("./assets/m1.txt".to_string());
    let mut map_string = String::new();
    let file = File::open(map_file).unwrap();
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut map_string).unwrap();

    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();
    let mut row_count = 0;
    let mut col_count = 0;

    for (y, row) in rows.iter().rev().enumerate() {
        row_count += 1;
        let columns: Vec<&str> = row.split(' ').collect();
        for (x, column) in columns.iter().enumerate() {
            col_count += 1;
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };

            // 0 player 1 floor 2 spot 3 wall 4 box
            match *column {
                "." => {
                    // floor
                    commands
                        .spawn(SpriteSheetComponents {
                            texture_atlas: texture_atlas_sheet,
                            scale: Scale(SCALE),
                            sprite: TextureAtlasSprite::new(1),
                            ..Default::default()
                        })
                        .with(pos.clone())
                        .with(Floor {});
                }
                "W" => {
                    // wall
                    commands
                        .spawn(SpriteSheetComponents {
                            texture_atlas: texture_atlas_sheet,
                            scale: Scale(SCALE),
                            sprite: TextureAtlasSprite::new(3),
                            translation: Translation::new(0.0, 0.0, 1.0),
                            ..Default::default()
                        })
                        .with(pos)
                        .with(Wall {})
                        .with(Immovable);
                }
                "P" => {
                    commands
                        // floor
                        .spawn(SpriteSheetComponents {
                            texture_atlas: texture_atlas_sheet,
                            scale: Scale(SCALE),
                            sprite: TextureAtlasSprite::new(1),
                            ..Default::default()
                        })
                        .with(pos.clone())
                        .with(Floor {})
                        // player
                        .spawn(SpriteSheetComponents {
                            texture_atlas: texture_atlas_player,
                            scale: Scale(SCALE),
                            translation: Translation::new(0.0, 0.0, 1.0),
                            ..Default::default()
                        })
                        .with(Timer::from_seconds(0.1, true))
                        .with(pos.clone())
                        .with(Player {
                            name: "player".to_string(),
                            step: 0,
                        });
                }
                "B" => {
                    commands
                        // floor
                        .spawn(SpriteSheetComponents {
                            texture_atlas: texture_atlas_sheet,
                            scale: Scale(SCALE),
                            sprite: TextureAtlasSprite::new(1),
                            ..Default::default()
                        })
                        .with(pos.clone())
                        .with(Floor {})
                        // box
                        .spawn(SpriteSheetComponents {
                            texture_atlas: texture_atlas_box_blue,
                            scale: Scale(SCALE),
                            // sprite: TextureAtlasSprite::new(4),
                            translation: Translation::new(0.0, 0.0, 1.0),
                            ..Default::default()
                        })
                        .with(Timer::from_seconds(0.5, true))
                        .with(pos.clone())
                        .with(Box {
                            sprite_ok: (texture_atlas_sheet, 4),
                        })
                        .with(Movable);
                }
                "S" => {
                    commands
                        // floor
                        .spawn(SpriteSheetComponents {
                            texture_atlas: texture_atlas_sheet,
                            scale: Scale(SCALE),
                            sprite: TextureAtlasSprite::new(1),
                            ..Default::default()
                        })
                        .with(pos.clone())
                        .with(Floor {})
                        // box spot
                        .spawn(SpriteSheetComponents {
                            texture_atlas: texture_atlas_sheet,
                            scale: Scale(SCALE),
                            sprite: TextureAtlasSprite::new(2),
                            translation: Translation::new(0.0, 0.0, 0.1),
                            ..Default::default()
                        })
                        .with(pos)
                        .with(BoxSpot {});
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }

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
