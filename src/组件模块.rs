use bevy::prelude::*;
use std::ops::{Add, Sub};

/// 用户
pub struct 玩家;

/// 箱子
pub struct 箱子 {
    pub sprite_ok: (Handle<TextureAtlas>, u32),
}

/// 目标点
pub struct 目标点 {
    pub 到达: bool,
}

/// 石头
// struct Stone {}

/// 墙
pub struct 墙体;

// 地板
pub struct 地板;

pub struct 可移动的;

pub struct 不可移动的;

// use std::cmp::Eq;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct 坐标 {
    pub x: i32,
    pub y: i32,
}

impl Add for 坐标 {
    type Output = 坐标;

    fn add(self, other: 坐标) -> 坐标 {
        坐标 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for 坐标 {
    type Output = 坐标;

    fn sub(self, other: 坐标) -> 坐标 {
        坐标 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
