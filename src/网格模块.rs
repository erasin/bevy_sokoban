use crate::状态模块::全局状态;
use bevy::prelude::*;

pub struct Grid(pub i32, pub i32);

#[derive(Default)]
pub struct 网格插件;

impl Plugin for 网格插件 {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid(10, 10))
            .add_system_set(SystemSet::on_enter(全局状态::游戏中).with_system(初始化处理));
    }
}

// 绘制网格
fn 初始化处理(
    mut commands: Commands,
    grid: Res<Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    数据: Res<全局数据>,
) {
    let col_count = grid.0;
    let row_count = grid.1;

    let line_color = materials.add(Color::hex("000000").unwrap().into());
    let def_width = 数据.缩放比例 * 数据.瓦片尺寸;
    let b_x = def_width * col_count as f32;
    let b_y = def_width * row_count as f32;
    let bounds = Vec2::new(b_x, b_y);

    for i in 0..=row_count {
        let j = i as f32;
        commands.spawn().insert_bundle(SpriteBundle {
            material: line_color.cast_weak(),
            transform: Transform::from_translation(Vec3::new(
                def_width * j - b_x / 2.0 - j,
                0.0,
                0.0,
            )),
            sprite: Sprite {
                size: Vec2::new(2.0, bounds.y),
                resize_mode: SpriteResizeMode::Automatic,
                ..Default::default()
            },
            ..Default::default()
        });
    }

    for i in 0..col_count {
        let j = i as f32;
        commands.spawn().insert_bundle(SpriteBundle {
            material: line_color.cast_weak(),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                def_width * j - b_y / 2.0 - j,
                0.0,
            )),
            sprite: Sprite {
                size: Vec2::new(bounds.x, 2.0),
                resize_mode: SpriteResizeMode::Automatic,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
