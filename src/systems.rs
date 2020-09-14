use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::{SCALE, TILED_WIDTH};
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    math::vec2,
    prelude::*,
    render::camera::Camera,
};
use std::collections::HashMap;
use std::collections::HashSet;

/// fsp 显示
pub fn fps_update_system(diagnostics: Res<Diagnostics>, mut query: Query<(&mut Text, &UIFPS)>) {
    for (mut text, _) in &mut query.iter() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

/// 镜头处理
pub fn camera_system(map: Res<Map>, mut query: Query<(&Camera, &mut Translation)>) {
    let height = map.height as f32 * TILED_WIDTH * SCALE;
    let width = map.width as f32 * TILED_WIDTH * SCALE;

    for (_, mut trans) in &mut query.iter() {
        trans.0.set_x(height / 2.0);
        trans.0.set_y(width / 2.0);
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
    map: Res<Map>,
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

    // println!("1 {:?} ", vol);
    if vol == Position::default() {
        return;
    }
    // println!("2 {:?}", vol);

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
            // println!("{} {}", per.name, per.step);
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
    mut events: ResMut<Events<MyEvent>>,
    mut box_entity: Query<(
        Entity,
        &Position,
        &Box,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    mut spot_entity: Query<(&Position, &mut BoxSpot)>,
) {
    for (ps, mut pse) in &mut spot_entity.iter() {
        if !pse.ok {
            for (e, pb, b, mut sprite, mut texture) in &mut box_entity.iter() {
                if ps == pb {
                    commands.insert_one(e, Immovable);
                    commands.remove_one::<Movable>(e);
                    sprite.index = b.sprite_ok.1;
                    *texture = b.sprite_ok.0;
                    pse.ok = true;
                    events.send(MyEvent::new(pb.x, pb.y));
                }
            }
        }
    }
}

/// 积分器
pub fn scoreboard_system(time: Res<Time>) {
    let _delta_seconds = f32::min(0.2, time.delta_seconds);
}

/// 事件监听
pub fn event_listener_system(
    time: Res<Time>,
    mut state: ResMut<MyEventListenerState>,
    events: Res<Events<MyEvent>>,
) {
    let _delta_seconds = f32::min(0.2, time.delta_seconds);

    for ev in state.reader.iter(&events) {
        // do something with `ev`
        println!("my event, {:?}", ev);
    }
}

// change map
pub fn map_system(mut commands: Commands, mut map: ResMut<Map>, input: Res<Input<KeyCode>>) {
    let mut map_file = "";
    if input.just_released(KeyCode::Key1) {
        map_file = "./assets/m1.txt";
    }
    if input.just_released(KeyCode::Key2) {
        map_file = "./assets/m2.txt";
    }
    if input.just_released(KeyCode::Key3) {
        map_file = "./assets/m3.txt";
    }
    if input.just_released(KeyCode::Key4) {
        map_file = "./assets/m4.txt";
    }
    if !map_file.is_empty() {
        println!("{}", map_file);
        *map = Map::load(map_file).unwrap();
        map.render(&mut commands);
    }
}

pub fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &Children,
        &UIBTN,
    )>,
    text_query: Query<&mut Text>,
) {
    for (_button, interaction, mut material, children, _) in &mut interaction_query.iter() {
        let mut text = text_query.get_mut::<Text>(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.value = "Press".to_string();
                *material = button_materials.pressed;
                println!("Press ok")
                // load map
            }
            Interaction::Hovered => {
                text.value = "Hover".to_string();
                *material = button_materials.hovered;
            }
            Interaction::None => {
                text.value = "Button".to_string();
                *material = button_materials.normal;
            }
        }
    }
}
