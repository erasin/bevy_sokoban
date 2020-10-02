use crate::components::*;
use crate::SCALE;
use bevy::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Default)]
pub struct Map {
    pub height: usize,
    pub width: usize,
    tides: Vec<Vec<String>>,
    texture_atlas_sheet: Handle<TextureAtlas>,
    texture_atlas_player: Handle<TextureAtlas>,
    texture_atlas_box_blue: Handle<TextureAtlas>,
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
        // {
        //     ..Default::default()
        // };

        map.tides = rows.clone();
        map.height = rows.len();
        if map.height > 0 {
            map.width = rows[0].len();
        }

        Ok(map)
    }
    pub fn set_atlas(
        &mut self,
        texture_atlas_sheet: Handle<TextureAtlas>,
        texture_atlas_player: Handle<TextureAtlas>,
        texture_atlas_box_blue: Handle<TextureAtlas>,
    ) {
        self.texture_atlas_sheet = texture_atlas_sheet;
        self.texture_atlas_player = texture_atlas_player;
        self.texture_atlas_box_blue = texture_atlas_box_blue;
    }

    /// 渲染处理
    pub fn render(&self, commands: &mut Commands) {
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
                                texture_atlas: self.texture_atlas_sheet,
                                transform: Transform::from_scale(SCALE),
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
                                texture_atlas: self.texture_atlas_sheet,
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
                                texture_atlas: self.texture_atlas_sheet,
                                transform: Transform::from_scale(SCALE),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .with(pos.clone())
                            .with(Floor {})
                            // player
                            .spawn(SpriteSheetComponents {
                                texture_atlas: self.texture_atlas_player,
                                transform: Transform::from_scale(SCALE)
                                    .with_translation(Vec3::new(0.0, 0.0, 1.0)),
                                ..Default::default()
                            })
                            .with(Timer::from_seconds(0.1, true))
                            .with(pos.clone())
                            .with(Player {
                                name: "player".to_string(),
                                step: 0,
                            });
                    }
                    "B" => {
                        commands
                            // floor
                            .spawn(SpriteSheetComponents {
                                texture_atlas: self.texture_atlas_sheet,
                                transform: Transform::from_scale(SCALE),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .with(pos.clone())
                            .with(Floor {})
                            // box
                            .spawn(SpriteSheetComponents {
                                texture_atlas: self.texture_atlas_box_blue,
                                transform: Transform::from_scale(SCALE)
                                    .with_translation(Vec3::new(0.0, 0.0, 2.0)),
                                ..Default::default()
                            })
                            .with(Timer::from_seconds(0.5, true))
                            .with(pos.clone())
                            .with(Box {
                                sprite_ok: (self.texture_atlas_sheet, 4),
                            })
                            .with(Movable);
                    }
                    "S" => {
                        commands
                            // floor
                            .spawn(SpriteSheetComponents {
                                texture_atlas: self.texture_atlas_sheet,
                                transform: Transform::from_scale(SCALE),
                                sprite: TextureAtlasSprite::new(1),
                                ..Default::default()
                            })
                            .with(pos.clone())
                            .with(Floor {})
                            // box spot
                            .spawn(SpriteSheetComponents {
                                texture_atlas: self.texture_atlas_sheet,
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

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}
