use crate::事件模块::地图加载事件;
use crate::数据模块::全局数据;
use crate::状态模块::标签;
use crate::{加载模块::纹理素材, 状态模块::全局状态, 组件模块::*};

use bevy::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Default)]
pub struct 地图插件;

impl Plugin for 地图插件 {
    fn build(&self, app: &mut App) {
        app.add_event::<地图加载事件>()
            .init_resource::<地图数据>()
            .add_system_set(
                SystemSet::on_update(全局状态::游戏中)
                    .with_system(快速加载处理)
                    .with_system(地图加载事件监听)
                    .with_system(缩放处理)
                    .label(标签::地图加载),
            );
    }
}

#[derive(Debug, Default)]
pub struct 地图数据 {
    pub height: usize,
    pub width: usize,
    元素: Vec<Vec<String>>,
    pub 目标数量: i32,
}

impl 地图数据 {
    /// 加载
    pub fn 加载<P: AsRef<Path>>(文件路径: P) -> anyhow::Result<Self> {
        let mut 内容缓存 = String::new();
        let file = File::open(文件路径)?;
        // file.read_to_string(buf)
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut 内容缓存).unwrap();

        let rows: Vec<Vec<String>> = 内容缓存
            .trim()
            .split("\n")
            .map(|x| x.trim().split(' ').map(|y| y.to_string()).collect())
            .collect();

        let mut map = 地图数据::default();

        map.height = rows.len();
        if map.height > 0 {
            map.width = rows[0].len();
        }
        map.元素 = rows;
        Ok(map)
    }

    /// 渲染处理
    pub fn 渲染处理(
        &mut self, mut 指令: Commands, 素材: Res<纹理素材>, 缩放比例: f32
    ) {
        for (y, row) in self.元素.iter().rev().enumerate() {
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
                            .insert(AnimationTimer(Timer::from_seconds(0.2, true)))
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
                            .insert(AnimationTimer(Timer::from_seconds(0.5, true)))
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

                        self.目标数量 += 1;
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

fn 快速加载处理(
    键位: Res<Input<KeyCode>>,
    mut 地图加载事件发送器: EventWriter<地图加载事件>,
) {
    let mut lv = 0;
    if 键位.just_released(KeyCode::Key1) {
        lv = 1;
    }
    if 键位.just_released(KeyCode::Key2) {
        lv = 2;
    }
    if 键位.just_released(KeyCode::Key3) {
        lv = 3;
    }
    if 键位.just_released(KeyCode::Key4) {
        lv = 4
    }

    if lv > 0 {
        地图加载事件发送器.send(地图加载事件(lv));
    }
}

fn 地图加载事件监听(
    mut 指令: Commands,
    mut 地图加载事件读取器: EventReader<地图加载事件>,
    mut 坐标实体: Query<Entity, With<坐标>>,
    mut 地图资源: ResMut<地图数据>,
    素材: Res<纹理素材>,
    mut 数据: ResMut<全局数据>,
) {
    let mut 地图文件 = String::new();
    let mut 地图编号 = 0;
    for ev in 地图加载事件读取器.iter() {
        if ev.0 > 0 && ev.0 < 5 {
            地图文件 = format!("./assets/m{}.txt", ev.0);
            地图编号 = ev.0;
        }
    }

    if !地图文件.is_empty() {
        // 判定文件
        坐标实体.for_each_mut(|e| {
            指令.entity(e).despawn_recursive();
        });
        数据.计步数 = 0;
        数据.踩点 = 0;
        数据.地图 = Some(地图编号);
        *地图资源 = 地图数据::加载(地图文件).unwrap();
        地图资源.渲染处理(指令, 素材, 数据.缩放比例);
    }
}

fn 缩放处理(mut 数据: ResMut<全局数据>, mut 按键: ResMut<Input<KeyCode>>) {
    if 按键.just_pressed(KeyCode::Equals) {
        数据.缩放比例 += 1.0;
        if 数据.缩放比例 == 0.0 {
            数据.缩放比例 = 1.0;
        }
        按键.reset(KeyCode::Equals);
    }
    if 按键.just_pressed(KeyCode::Minus) {
        数据.缩放比例 -= 1.0;
        if 数据.缩放比例 == 0.0 {
            数据.缩放比例 = -1.0;
        }
        按键.reset(KeyCode::Minus);
    }
}
