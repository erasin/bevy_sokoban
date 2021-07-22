use crate::镜头模块::*;
use bevy::prelude::*;

#[derive(Debug)]
pub struct 移动事件(i32, i32);

impl 移动事件 {
    pub fn new(x: i32, y: i32) -> Self {
        移动事件(x, y)
    }
}

/// 事件监听
pub fn 事件监听处理(
    系统时间: Res<Time>,
    mut 镜头数据: ResMut<镜头数据>,
    mut 移动事件读取器: EventReader<移动事件>,
) {
    let _delta_seconds = f32::min(0.2, 系统时间.delta_seconds());

    for ev in 移动事件读取器.iter() {
        // do something with `ev`
        println!("my event, {:?}", *ev);
        镜头数据.状态 = 镜头状态::抖动;
    }
}
