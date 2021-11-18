use crate::缩放比例;
use crate::{加载模块::纹理素材, 状态模块::全局状态, 组件模块::*};

use bevy::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Default)]
pub struct 地图插件;

impl Plugin for 地图插件 {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<地图数据>().add_system_set(
            SystemSet::on_update(全局状态::游戏中)
                .with_system(加载处理.system())
                .label("loadmap"),
        );
    }
}

#[derive(Debug, Default)]
pub struct 地图数据 {
    pub height: usize,
    pub width: usize,
    tides: Vec<Vec<String>>,
}

impl 地图数据 {
    /// 加载
    pub fn 加载<P: AsRef<Path>>(filepath: P) -> anyhow::Result<Self> {
        let mut map_string = String::new();
        let file = File::open(filepath)?;
        // file.read_to_string(buf)
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut map_string).unwrap();

        let rows: Vec<Vec<String>> = map_string
            .trim()
            .split("\n")
            .map(|x| x.trim().split(' ').map(|y| y.to_string()).collect())
            .collect();

        let mut map = 地图数据::default();

        map.tides = rows.clone();
        map.height = rows.len();
        if map.height > 0 {
            map.width = rows[0].len();
        }

        Ok(map)
    }

    /// 渲染处理
    pub fn 渲染处理(&self, mut 指令: Commands, 素材: Res<纹理素材>) {
        for (y, row) in self.tides.iter().rev().enumerate() {
            for (x, column) in row.iter().enumerate() {
                let 当前坐标 = 坐标 {
                    x: x as i32,
                    y: y as i32,
                };

                // 0 player 1 floor 2 spot 3 wall 4 box
                match column.as_str() {
                    "." => {
                        // floor
                        指令
                            .spawn()
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: 素材.纹理表.as_weak(),
                                transform: Transform::from_scale(Vec3::new(
                                    缩放比例,
                                    缩放比例,
                                    1.0,
                                )),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .insert(当前坐标.clone())
                            .insert(地板);
                    }
                    "W" => {
                        // wall
                        指令
                            .spawn()
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: 素材.纹理表.as_weak(),
                                transform: Transform {
                                    translation: Vec3::new(0.0, 0.0, 1.0),
                                    rotation: Quat::IDENTITY,
                                    scale: Vec3::new(缩放比例, 缩放比例, 1.0),
                                },
                                sprite: TextureAtlasSprite::new(3),
                                ..Default::default()
                            })
                            .insert(当前坐标)
                            .insert(墙体 {})
                            .insert(不可移动的);
                    }
                    "P" => {
                        指令
                            // floor
                            .spawn()
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: 素材.纹理表.as_weak(),
                                transform: Transform::from_scale(Vec3::new(
                                    缩放比例,
                                    缩放比例,
                                    1.0,
                                )),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .insert(当前坐标.clone())
                            .insert(地板);

                        指令
                            .spawn()
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: 素材.用户.as_weak(),
                                transform: Transform {
                                    translation: Vec3::new(0.0, 0.0, 1.0),
                                    rotation: Quat::IDENTITY,
                                    scale: Vec3::new(缩放比例, 缩放比例, 1.0),
                                },
                                ..Default::default()
                            })
                            .insert(Timer::from_seconds(0.2, true))
                            .insert(当前坐标.clone())
                            .insert(玩家);
                    }
                    "B" => {
                        // floor
                        指令
                            .spawn()
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: 素材.纹理表.as_weak(),
                                transform: Transform::from_scale(Vec3::new(
                                    缩放比例,
                                    缩放比例,
                                    1.0,
                                )),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .insert(当前坐标.clone())
                            .insert(地板);

                        // box
                        指令
                            .spawn()
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: 素材.蓝箱子.as_weak(),
                                transform: Transform {
                                    translation: Vec3::new(0.0, 0.0, 2.0),
                                    rotation: Quat::IDENTITY,
                                    scale: Vec3::new(缩放比例, 缩放比例, 1.0),
                                },
                                ..Default::default()
                            })
                            .insert(Timer::from_seconds(0.5, true))
                            .insert(当前坐标.clone())
                            .insert(箱子 {
                                sprite_ok: (素材.纹理表.as_weak(), 4),
                            })
                            .insert(可移动的);
                    }
                    "S" => {
                        // floor
                        指令
                            .spawn()
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: 素材.纹理表.as_weak(),
                                transform: Transform::from_scale(Vec3::new(
                                    缩放比例,
                                    缩放比例,
                                    1.0,
                                )),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .insert(当前坐标.clone())
                            .insert(地板);

                        // box spot
                        指令
                            .spawn()
                            .insert_bundle(SpriteSheetBundle {
                                texture_atlas: 素材.纹理表.as_weak(),
                                sprite: TextureAtlasSprite::new(2),
                                transform: Transform {
                                    translation: Vec3::new(0.0, 0.0, 0.1),
                                    rotation: Quat::IDENTITY,
                                    scale: Vec3::new(缩放比例, 缩放比例, 1.0),
                                },
                                ..Default::default()
                            })
                            .insert(当前坐标)
                            .insert(目标点 { 到达: false });
                    }
                    "-" => (),
                    c => panic!("unrecognized map item {}", c),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::地图数据;

    #[test]
    fn test_map_load() {
        let m = 地图数据::加载("assets/m3.txt").unwrap();
        assert_eq!(m.height, 8);
        assert_eq!(m.width, 9);
    }
}

/// 加载地图
fn 加载处理(
    mut 指令: Commands,
    mut 地图资源: ResMut<地图数据>,
    键位: Res<Input<KeyCode>>,
    素材: Res<纹理素材>,
    pos_query: Query<(Entity, &坐标)>,
) {
    let mut map_file = "";
    if 键位.just_released(KeyCode::Key1) {
        map_file = "./assets/m1.txt";
    }
    if 键位.just_released(KeyCode::Key2) {
        map_file = "./assets/m2.txt";
    }
    if 键位.just_released(KeyCode::Key3) {
        map_file = "./assets/m3.txt";
    }
    if 键位.just_released(KeyCode::Key4) {
        map_file = "./assets/m4.txt";
    }

    if !map_file.is_empty() {
        // 清理
        for (e, _) in pos_query.iter() {
            指令.entity(e).despawn()
        }

        *地图资源 = 地图数据::加载(map_file).unwrap();
        地图资源.渲染处理(指令, 素材);
    }
}
