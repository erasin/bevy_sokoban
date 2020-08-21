use bevy::{math::vec2, prelude::*, render::camera::Camera, render::pass::ClearColor};

const TILED_WIDTH: f32 = 32.0;
const SCALE: f32 = 2.0;

/// 用户
struct Player {}

/// 箱子
struct Box {
    sprite_ok: (Handle<TextureAtlas>, u32),
}

/// 目标点
struct BoxSpot {}

/// 石头
// struct Stone {}

/// 墙
struct Wall {}

// 地板
struct Floor {}

struct Movable;

struct Immovable;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    pub x: f32,
    pub y: f32,
}

use std::ops::{Add, Sub};

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn setup(
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
            },
            ..Default::default()
        });

    // 地图加载
    let map_string: &str = std::include_str!("../assets/m1.txt");

    // let map_string: &str = "
    //     N N W W W W W W
    //     W W W . . . . W
    //     W . . . B . . W
    //     W . . . . . . W
    //     W . P . . . . W
    //     W . . . . . . W
    //     W . . S . . . W
    //     W . . . . . . W
    //     W W W W W W W W
    //     ";

    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().rev().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();
        for (x, column) in columns.iter().enumerate() {
            let pos = Position {
                x: x as f32,
                y: y as f32,
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
                        .with(Timer::from_seconds(0.1))
                        .with(pos.clone())
                        .with(Player {});
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
                        .with(Timer::from_seconds(0.5))
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

    // clamp the timestep to stop the ball from escaping when the game starts
}

// 镜头处理
fn camera_system(mut query: Query<(&Camera, &mut Translation)>) {
    for (_, mut trans) in &mut query.iter() {
        trans.0.set_x(300.0);
        trans.0.set_y(300.0);
    }
}

// 动画效果
fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            timer.reset();
        }
    }
}

// 坐标转化
fn position_system(mut query: Query<(&mut Position, &mut Translation)>) {
    for (pos, mut trans) in &mut query.iter() {
        let start = vec2(trans.0.x(), trans.0.y());
        let end = start.lerp(
            vec2(pos.x * TILED_WIDTH * SCALE, pos.y * TILED_WIDTH * SCALE),
            0.35,
        );
        trans.0.set_x(end.x());
        trans.0.set_y(end.y());
    }
}

//  移动
fn player_movement_system(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Position, &Player)>,
    mut immovable: Query<(&Position, &Immovable)>,
    mut moveable: Query<(&mut Position, &Movable)>,
    // mut camera: Query<(&mut Translation, &Camera)>,
) {
    let _delta_seconds = f32::min(0.2, time.delta_seconds);

    for (mut pos, _) in &mut player.iter() {
        let mut vol = Position { x: 0.0, y: 0.0 };

        if input.just_released(KeyCode::Up) {
            vol.y = 1.0;
        }
        if input.just_released(KeyCode::Down) {
            vol.y = -1.0;
        }
        if input.just_released(KeyCode::Right) {
            vol.x = 1.0;
        }
        if input.just_released(KeyCode::Left) {
            vol.x = -1.0;
        }

        // 下个位置
        let mut p2 = pos.clone();
        p2.x = p2.x + vol.x;
        p2.y = p2.y + vol.y;

        // 下下位置
        let mut p3 = pos.clone();
        p3.x = p2.x + vol.x;
        p3.y = p2.y + vol.y;

        let mut p3_move = true;

        for (pos_im, _) in &mut immovable.iter() {
            if pos_im.x == p2.x && pos_im.y == p2.y {
                vol = Position { x: 0.0, y: 0.0 };
            }
            if pos_im.x == p3.x && pos_im.y == p3.y {
                p3_move = false;
            }
        }

        if vol.x != 0.0 || vol.y != 0.0 {
            for (mut pos_mo, _) in &mut moveable.iter() {
                if pos_mo.x == p2.x && pos_mo.y == p2.y {
                    if p3_move {
                        pos_mo.x = pos_mo.x + vol.x;
                        pos_mo.y = pos_mo.y + vol.y;
                    } else {
                        vol = Position { x: 0.0, y: 0.0 };
                    }
                }
            }

            // 移动
            pos.x = pos.x + vol.x;
            pos.y = pos.y + vol.y;
        }

        // 镜头跟随用户 camera follow the player
        // for (mut trans, _) in &mut camera.iter() {
        //     trans.0.set_x(pos.0 * TILED_WIDTH * SCALE);
        //     trans.0.set_y(pos.1 * TILED_WIDTH * SCALE);
        // }
    }
}

fn box_spot_system(
    mut box_entity: Query<(
        &Position,
        &Box,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    mut spot_entity: Query<(&Position, &BoxSpot)>,
) {
    for (ps, _) in &mut spot_entity.iter() {
        for (pb, b, mut sprite, mut texture) in &mut box_entity.iter() {
            if ps.x == pb.x && ps.y == pb.y {
                sprite.index = b.sprite_ok.1;
                *texture = b.sprite_ok.0;
            }
        }
    }
}

fn scoreboard_system(time: Res<Time>) {
    let _delta_seconds = f32::min(0.2, time.delta_seconds);
}

fn event_listener_system(time: Res<Time>) {
    let _delta_seconds = f32::min(0.2, time.delta_seconds);
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "sokoban!".to_string(),
            width: 800,
            height: 800,
            vsync: true,
            // resizable: false,
            // mode: WindowMode::Fullscreen { use_size: false },
            ..Default::default()
        })
        .add_default_plugins()
        .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_startup_system(setup.system())
        .add_system_to_stage(stage::FIRST, camera_system.system())
        .add_system(animate_sprite_system.system())
        .add_system(player_movement_system.system())
        .add_system(box_spot_system.system())
        .add_system(position_system.system())
        .add_system(scoreboard_system.system())
        .add_system(event_listener_system.system())
        .run();
}
