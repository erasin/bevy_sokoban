use bevy::prelude::*;

use crate::resources::*;
use crate::TILED_WIDTH;

pub struct Grid(pub i32, pub i32);

#[derive(Default)]
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_system.system())
            .add_resource(Grid(10, 10));
    }
}

// 绘制网格
fn setup_system(
    mut commands: Commands,
    grid: Res<Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let col_count = grid.0;
    let row_count = grid.1;

    let line_color = materials.add(Color::hex("000000").unwrap().into());
    let bounds = Vec2::new(400.0, 500.0);
    let def_width = TILED_WIDTH;

    for i in 0..=row_count {
        let j = i as f32;
        commands.spawn(SpriteComponents {
            material: line_color,
            transform: Transform::from_translation(Vec3::new(def_width * j, 0.0, 0.0)),
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
            transform: Transform::from_translation(Vec3::new(0.0, def_width * j, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x(), 2.0),
                resize_mode: SpriteResizeMode::Automatic,
            },
            ..Default::default()
        });
    }
}
