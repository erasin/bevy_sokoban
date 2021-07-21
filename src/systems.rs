use crate::data::*;
use crate::events::*;
use crate::loading::AudioAssets;
use crate::map::*;
use crate::{components::*, state::GameState};
use crate::{SCALE, TILED_WIDTH};
use bevy::{math::vec2, prelude::*};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<MyEvent>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .after("loadmap")
                .with_system(animate_sprite_system.system())
                .with_system(box_spot_system.system())
                .after("action")
                .with_system(position_system.system())
                .with_system(scoreboard_system.system())
                .with_system(event_listener_system.system()),
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
pub fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>), With<Player>>,
) {
    for (timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

/// 坐标转化
pub fn position_system(map: Res<Map>, mut query: Query<(&mut Position, &mut Transform)>) {
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

use crate::actions::Actions;

///  移动
pub fn player_movement_system(
    // time: Res<Time>,
    actions: Res<Actions>,
    radio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut data: ResMut<GameData>,
    // mut player_query: Query<(Entity, &mut Position, &mut Player)>,
    // immovable_query: Query<(Entity, &Position, &Immovable)>,
    // mut moveable_query: Query<(Entity, &mut Position, &Movable), Without<Player>>,
    mut query: QuerySet<(
        Query<(Entity, &mut Position, &mut Player)>,
        Query<(Entity, &Position, &Immovable)>,
        Query<(Entity, &mut Position, &Movable), Without<Player>>,
    )>,
) {
    // let _delta_seconds = f32::min(0.2, time.delta_seconds);

    let mut vol = Position { x: 0, y: 0 };

    if actions.player_movement.is_none() {
        return;
    }
    vol.x = actions.player_movement.unwrap().x as i32;
    vol.y = actions.player_movement.unwrap().y as i32;

    // if input.just_released(KeyCode::Up) {
    //     vol.y = 1;
    // }
    // if input.just_released(KeyCode::Down) {
    //     vol.y = -1;
    // }
    // if input.just_released(KeyCode::Right) {
    //     vol.x = 1;
    // }
    // if input.just_released(KeyCode::Left) {
    //     vol.x = -1;
    // }

    // println!("1 {:?} ", vol);
    if vol == Position::default() {
        return;
    }
    // println!("2 {:?}", vol);

    // 移动链对象
    let mut to_move = HashSet::new();

    // 所有可移动
    let mov: HashMap<(i32, i32), u32> = query
        .q2_mut()
        .iter_mut()
        .map(|(e, p, _)| ((p.x, p.y), e.id()))
        .collect::<HashMap<_, _>>();

    // 所有不可移动
    let immvo: HashMap<(i32, i32), u32> = query
        .q1()
        .iter()
        .map(|(e, p, _)| ((p.x, p.y), e.id()))
        .collect::<HashMap<_, _>>();

    for (entity, mut pos, mut _per) in query.q0_mut().iter_mut() {
        to_move.insert(entity.id());

        // 移动方向链存储器
        let (start, end, is_x) = match vol {
            Position { x: 0, y } => {
                let len = if y > 0 { map.height as i32 } else { 0 };
                ((pos.y + y) as i32, len, false)
            }
            Position { x, y: 0 } => {
                let len = if x > 0 { map.width as i32 } else { 0 };
                ((pos.x + x) as i32, len, true)
            }
            _ => (0, 0, false),
        };

        // println!("4 {:?} ", vol);

        // println!("3 {} {} {}", start, end, is_x);

        // 缓存
        let range = if start < end {
            (start..=end).collect::<Vec<_>>()
        } else {
            (end..=start).rev().collect::<Vec<_>>()
        };

        for x_y in range {
            let p2 = if is_x { (x_y, pos.y) } else { (pos.x, x_y) };

            // println!("5 -> {:?} ", p2);

            match mov.get(&p2) {
                Some(id) => to_move.insert(*id),
                None => {
                    radio.play(audio_assets.audio_wall.as_weak());
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
            data.step += 1;
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
pub fn box_spot_system(
    mut commands: Commands,
    mut data: ResMut<GameData>,
    mut events: EventWriter<MyEvent>,
    mut box_entity: Query<(
        Entity,
        &Position,
        &Box,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    mut spot_entity: Query<(&Position, &mut BoxSpot)>,
) {
    for (ps, mut pse) in spot_entity.iter_mut() {
        if !pse.ok {
            for (e, pb, b, mut sprite, mut texture) in box_entity.iter_mut() {
                if ps == pb {
                    commands.entity(e).remove::<Movable>().insert(Immovable);
                    // commands.remove_one::<Movable>(e);
                    // commands.insert_one(e, Immovable);
                    sprite.index = b.sprite_ok.1;
                    *texture = b.sprite_ok.0.as_weak();
                    pse.ok = true;
                    data.spot += 1;
                    events.send(MyEvent::new(pb.x, pb.y));
                }
            }
        }
    }
}

/// 积分器
pub fn scoreboard_system(time: Res<Time>) {
    let _delta_seconds = f32::min(0.2, time.delta_seconds());
}
