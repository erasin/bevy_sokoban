use crate::事件模块::*;
use crate::加载模块::音频素材;
use crate::地图模块::*;
use crate::数据模块::*;
use crate::{状态模块::全局状态, 组件模块::*};
use crate::{SCALE, TILED_WIDTH};

use bevy::{math::vec2, prelude::*};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct 行为组件;

impl Plugin for 行为组件 {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<移动事件>().add_system_set(
            SystemSet::on_update(全局状态::游戏中)
                .after("loadmap")
                .with_system(动画效果处理.system())
                .with_system(箱子移动到目标处理.system())
                .after("action")
                .with_system(坐标转化处理.system())
                .with_system(玩家移动处理.system())
                .with_system(积分器处理.system())
                .with_system(事件监听处理.system()),
        );
    }
}

/// 镜头处理
// pub fn camera_system(map: Res<Map>, mut query: Query<(&Camera, &mut Transform)>) {
//     let height = map.height as f32 * TILED_WIDTH * SCALE;
//     let width = map.width as f32 * TILED_WIDTH * SCALE;

//     for (_, mut trans) in &mut query.iter() {
//         // 相机 z 高度位置需要高于要显示的对象
//         trans.set_translation(Vec3::new(height / 2.0, width / 2.0, 2.0));
//     }
// }

/// 动画效果
pub fn 动画效果处理(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>), With<玩家>>,
) {
    for (timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

/// 坐标转化
pub fn 坐标转化处理(map: Res<地图数据>, mut query: Query<(&mut 坐标, &mut Transform)>) {
    let height = map.height as f32 * TILED_WIDTH * SCALE;
    let width = map.width as f32 * TILED_WIDTH * SCALE;

    let x = width / 2.0;
    let y = height / 2.0;

    for (pos, mut trans) in query.iter_mut() {
        let t = trans.translation.clone();
        let start = vec2(t.x, t.y);
        let end = start.lerp(
            vec2(
                pos.x as f32 * SCALE * TILED_WIDTH - x,
                pos.y as f32 * SCALE * TILED_WIDTH - y,
            ),
            0.35,
        );
        trans.translation.x = end.x;
        trans.translation.y = end.y;
    }
}

use crate::行为模块::动作集;

///  移动  
pub fn 玩家移动处理(
    // time: Res<Time>,
    动作: Res<动作集>,
    音频: Res<Audio>,
    音频资源: Res<音频素材>,
    // input: Res<Input<KeyCode>>,
    地图: Res<地图数据>,
    mut 数据: ResMut<全局数据>,
    // mut player_query: Query<(Entity, &mut Position, &mut Player)>,
    // immovable_query: Query<(Entity, &Position, &Immovable)>,
    // mut moveable_query: Query<(Entity, &mut Position, &Movable), Without<Player>>,
    mut query: QuerySet<(
        Query<(Entity, &mut 坐标, &mut 玩家)>,
        Query<(Entity, &坐标, &不可移动的)>,
        Query<(Entity, &mut 坐标, &可移动的), Without<玩家>>,
    )>,
) {
    // let _delta_seconds = f32::min(0.2, time.delta_seconds);

    let mut vol = 坐标 { x: 0, y: 0 };

    if 动作.用户移动向量.is_none() {
        return;
    }
    vol.x = 动作.用户移动向量.unwrap().x as i32;
    vol.y = 动作.用户移动向量.unwrap().y as i32;

    if vol.x == vol.y && vol.y == 0 {
        return;
    }

    // 移动链对象
    let mut to_move = HashSet::new();

    // 所有可移动
    let 所有可移动对象: HashMap<(i32, i32), u32> = query
        .q2_mut()
        .iter_mut()
        .map(|(e, p, _)| ((p.x, p.y), e.id()))
        .collect::<HashMap<_, _>>();

    // 所有不可移动
    let 所有不可移动对象: HashMap<(i32, i32), u32> = query
        .q1()
        .iter()
        .map(|(e, p, _)| ((p.x, p.y), e.id()))
        .collect::<HashMap<_, _>>();

    for (玩家实体, mut 玩家坐标, mut _per) in query.q0_mut().iter_mut() {
        to_move.insert(玩家实体.id());

        // 移动方向链存储器
        let (start, end, is_x) = match &vol {
            &坐标 { x: 0, y } => {
                let len = if y > 0 { 地图.height as i32 } else { 0 };
                ((玩家坐标.y + y) as i32, len, false)
            }
            &坐标 { x, y: 0 } => {
                let len = if x > 0 { 地图.width as i32 } else { 0 };
                ((玩家坐标.x + x) as i32, len, true)
            }
            _ => (0, 0, false),
        };

        // 缓存
        let range = if start < end {
            (start..=end).collect::<Vec<_>>()
        } else {
            (end..=start).rev().collect::<Vec<_>>()
        };

        for x_y in range {
            let p2 = if is_x {
                (x_y, 玩家坐标.y)
            } else {
                (玩家坐标.x, x_y)
            };

            // println!("5 -> {:?} ", p2);

            match 所有可移动对象.get(&p2) {
                Some(id) => to_move.insert(*id),
                None => {
                    音频.play(音频资源.audio_wall.as_weak());
                    // 查询不可移动，清空队列
                    match 所有不可移动对象.get(&p2) {
                        Some(_) => to_move.clear(),
                        None => break,
                    }
                    break;
                }
            };
        }

        // 移动用户
        if to_move.remove(&玩家实体.id()) {
            *玩家坐标 = *玩家坐标 + vol;
            数据.计步数 += 1;
            // println!("{} {}", per.name, per.step);
        }
    }

    // 移动移动对象
    for (e, mut pos, _) in query.q2_mut().iter_mut() {
        if to_move.remove(&e.id()) {
            *pos = *pos + vol;
        }
    }
}

// 完成处理
pub fn 箱子移动到目标处理(
    mut 指令: Commands,
    mut 数据: ResMut<全局数据>,
    mut 移动事件发送器: EventWriter<移动事件>,
    mut 箱子实体集合: Query<(
        Entity,
        &坐标,
        &箱子,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    mut 目标位置集合: Query<(&坐标, &mut 目标点)>,
) {
    for (ps, mut pse) in 目标位置集合.iter_mut() {
        if !pse.到达 {
            for (e, pb, b, mut sprite, mut texture) in 箱子实体集合.iter_mut() {
                if ps == pb {
                    指令.entity(e).remove::<可移动的>().insert(不可移动的);
                    // commands.remove_one::<Movable>(e);
                    // commands.insert_one(e, Immovable);
                    sprite.index = b.sprite_ok.1;
                    *texture = b.sprite_ok.0.as_weak();
                    pse.到达 = true;
                    数据.踩点 += 1;
                    移动事件发送器.send(移动事件::new(pb.x, pb.y));
                }
            }
        }
    }
}

/// 积分器
pub fn 积分器处理(系统时间: Res<Time>) {
    let _delta_seconds = f32::min(0.2, 系统时间.delta_seconds());
}
