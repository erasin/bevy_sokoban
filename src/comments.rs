use bevy::prelude::*;
use std::ops::{Add, Sub};

/// 用户
pub struct Player {
    pub name: String,
    pub step: i32,
}

/// 箱子
pub struct Box {
    pub sprite_ok: (Handle<TextureAtlas>, u32),
}

/// 目标点
pub struct BoxSpot {}

/// 石头
// struct Stone {}

/// 墙
pub struct Wall {}

// 地板
pub struct Floor {}

pub struct Movable;

pub struct Immovable;

// use std::cmp::Eq;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
