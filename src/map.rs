use crate::components::*;
use crate::resources::*;
use crate::SCALE;

use bevy::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Default)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Map>()
            .add_system(reload_system.system());
    }
}

#[derive(Debug, Default)]
pub struct Map {
    pub height: usize,
    pub width: usize,
    tides: Vec<Vec<String>>,
}

impl Map {
    /// 加载
    pub fn load<P: AsRef<Path>>(filepath: P) -> anyhow::Result<Self> {
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

        let mut map = Map::default();

        map.tides = rows.clone();
        map.height = rows.len();
        if map.height > 0 {
            map.width = rows[0].len();
        }

        Ok(map)
    }

    /// 渲染处理
    pub fn render(&self, commands: &mut Commands, resource: Res<ResourceData>) {
        for (y, row) in self.tides.iter().rev().enumerate() {
            for (x, column) in row.iter().enumerate() {
                let pos = Position {
                    x: x as i32,
                    y: y as i32,
                };

                // 0 player 1 floor 2 spot 3 wall 4 box
                match column.as_str() {
                    "." => {
                        // floor
                        commands
                            .spawn(SpriteSheetComponents {
                                texture_atlas: resource.texture_atlas_sheet,
                                transform: Transform::from_scale(SCALE),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .with(pos.clone())
                            .with(Floor);
                    }
                    "W" => {
                        // wall
                        commands
                            .spawn(SpriteSheetComponents {
                                texture_atlas: resource.texture_atlas_sheet,
                                transform: Transform::from_scale(SCALE)
                                    .with_translation(Vec3::new(0.0, 0.0, 1.0)),
                                sprite: TextureAtlasSprite::new(3),
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
                                texture_atlas: resource.texture_atlas_sheet,
                                transform: Transform::from_scale(SCALE),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .with(pos.clone())
                            .with(Floor)
                            // player
                            .spawn(SpriteSheetComponents {
                                texture_atlas: resource.texture_atlas_player,
                                transform: Transform::from_scale(SCALE)
                                    .with_translation(Vec3::new(0.0, 0.0, 1.0)),
                                ..Default::default()
                            })
                            .with(Timer::from_seconds(0.2, true))
                            .with(pos.clone())
                            .with(Player);
                    }
                    "B" => {
                        commands
                            // floor
                            .spawn(SpriteSheetComponents {
                                texture_atlas: resource.texture_atlas_sheet,
                                transform: Transform::from_scale(SCALE),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .with(pos.clone())
                            .with(Floor)
                            // box
                            .spawn(SpriteSheetComponents {
                                texture_atlas: resource.texture_atlas_box_blue,
                                transform: Transform::from_scale(SCALE)
                                    .with_translation(Vec3::new(0.0, 0.0, 2.0)),
                                ..Default::default()
                            })
                            .with(Timer::from_seconds(0.5, true))
                            .with(pos.clone())
                            .with(Box {
                                sprite_ok: (resource.texture_atlas_sheet, 4),
                            })
                            .with(Movable);
                    }
                    "S" => {
                        commands
                            // floor
                            .spawn(SpriteSheetComponents {
                                texture_atlas: resource.texture_atlas_sheet,
                                transform: Transform::from_scale(SCALE),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .with(pos.clone())
                            .with(Floor)
                            // box spot
                            .spawn(SpriteSheetComponents {
                                texture_atlas: resource.texture_atlas_sheet,
                                sprite: TextureAtlasSprite::new(2),
                                transform: Transform::from_scale(SCALE)
                                    .with_translation(Vec3::new(0.0, 0.0, 0.1)),
                                ..Default::default()
                            })
                            .with(pos)
                            .with(BoxSpot { ok: false });
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
    use super::Map;

    #[test]
    fn test_map_load() {
        let m = Map::load("assets/m3.txt").unwrap();
        assert_eq!(m.height, 8);
        assert_eq!(m.width, 9);
    }
}

// change map
fn reload_system(
    mut commands: Commands,
    mut map: ResMut<Map>,
    input: Res<Input<KeyCode>>,
    resource: Res<ResourceData>,
    mut pos_query: Query<(Entity, &Position)>,
) {
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
        // 清理
        for (e, _) in &mut pos_query.iter() {
            commands.despawn(e);
        }

        *map = Map::load(map_file).unwrap();
        map.render(&mut commands, resource);
    }
}
