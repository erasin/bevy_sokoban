use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    math::vec2,
    prelude::*,
    render::camera::Camera,
};

use crate::comments::*;
use crate::{SCALE, TILED_WIDTH};
use std::collections::HashMap;
use std::collections::HashSet;

/// fsp 显示
pub fn fps_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

/// 镜头处理
pub fn camera_system(mut query: Query<(&Camera, &mut Translation)>) {
    for (_, mut trans) in &mut query.iter() {
        trans.0.set_x(300.0);
        trans.0.set_y(300.0);
    }
}

/// 动画效果
pub fn animate_sprite_system(
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

/// 坐标转化
pub fn position_system(mut query: Query<(&mut Position, &mut Translation)>) {
    for (pos, mut trans) in &mut query.iter() {
        let start = vec2(trans.0.x(), trans.0.y());
        let end = start.lerp(
            vec2(
                pos.x as f32 * SCALE * TILED_WIDTH,
                pos.y as f32 * SCALE * TILED_WIDTH,
            ),
            0.35,
        );
        trans.0.set_x(end.x());
        trans.0.set_y(end.y());
    }
}

///  移动
pub fn player_movement_system(
    // time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut Position, &mut Player)>,
    mut immovable_query: Query<(Entity, &Position, &Immovable)>,
    mut moveable_query: Query<Without<Player, (Entity, &mut Position, &Movable)>>,
    // mut camera: Query<(&mut Translation, &Camera)>,
) {
    // let _delta_seconds = f32::min(0.2, time.delta_seconds);

    let mut vol = Position { x: 0, y: 0 };

    if input.just_released(KeyCode::Up) {
        vol.y = 1;
    }
    if input.just_released(KeyCode::Down) {
        vol.y = -1;
    }
    if input.just_released(KeyCode::Right) {
        vol.x = 1;
    }
    if input.just_released(KeyCode::Left) {
        vol.x = -1;
    }

    println!("1 {:?} ", vol);
    if vol == Position::default() {
        return;
    }
    println!("2 {:?}", vol);

    // 移动链对象
    let mut to_move = HashSet::new();

    for (entity, mut pos, mut per) in &mut player_query.iter() {
        to_move.insert(entity.id());

        // 所有可移动
        let mov: HashMap<(i32, i32), u128> = moveable_query
            .iter()
            .iter()
            .map(|t| ((t.1.x, t.1.y), t.0.id()))
            .collect::<HashMap<_, _>>();

        // 所有不可移动
        let immvo: HashMap<(i32, i32), u128> = immovable_query
            .iter()
            .iter()
            .map(|t| ((t.1.x, t.1.y), t.0.id()))
            .collect::<HashMap<_, _>>();

        // 移动方向链存储器
        let (start, end, is_x) = match vol {
            Position { x: 0, y } => {
                let len = if y > 0 { 10 } else { 0 };
                ((pos.y + y) as i32, len, false)
            }
            Position { x, y: 0 } => {
                let len = if x > 0 { 10 } else { 0 };
                ((pos.x + x) as i32, len, true)
            }
            _ => (0, 0, false),
        };

        println!("4 {:?} ", vol);

        println!("3 {} {} {}", start, end, is_x);

        // 缓存
        let range = if start < end {
            (start..=end).collect::<Vec<_>>()
        } else {
            (end..=start).rev().collect::<Vec<_>>()
        };

        for x_y in range {
            let p2 = if is_x { (x_y, pos.y) } else { (pos.x, x_y) };

            println!("5 -> {:?} ", p2);

            match mov.get(&p2) {
                Some(id) => to_move.insert(*id),
                None => {
                    // 查询不可移动，清空队列
                    match immvo.get(&p2) {
                        Some(_) => to_move.clear(),
                        None => break,
                    }
                    break;
                }
            };
        }

        // 移动用户
        if to_move.remove(&entity.id()) {
            *pos = *pos + vol;
            per.step += 1;
            println!("{} {}", per.name, per.step);
        }
    }

    // 移动移动对象
    for (e, mut pos, _) in &mut moveable_query.iter() {
        if to_move.remove(&e.id()) {
            *pos = *pos + vol;
        }
    }
}

// 完成处理
pub fn box_spot_system(
    mut commands: Commands,
    mut box_entity: Query<(
        Entity,
        &Position,
        &Box,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    mut spot_entity: Query<(&Position, &BoxSpot)>,
) {
    for (ps, _) in &mut spot_entity.iter() {
        for (e, pb, b, mut sprite, mut texture) in &mut box_entity.iter() {
            if ps == pb {
                // commands.insert_one(e, Immovable);
                // commands.remove_one::<Movable>(e);
                sprite.index = b.sprite_ok.1;
                *texture = b.sprite_ok.0;
            }
        }
    }
}

/// 积分器
pub fn scoreboard_system(time: Res<Time>) {
    let _delta_seconds = f32::min(0.2, time.delta_seconds);
}

/// 事件监听
pub fn event_listener_system(time: Res<Time>) {
    let _delta_seconds = f32::min(0.2, time.delta_seconds);
}
