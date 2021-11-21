use bevy::prelude::Plugin;

use crate::状态模块::全局状态;

#[derive(Clone)]
pub struct 全局数据 {
    pub 用户名: String,
    pub 计步数: i32,
    pub 踩点: i32,
    pub 地图: Option<i32>,
    pub 缩放比例: f32,
    pub 瓦片尺寸: f32,
}

impl Default for 全局数据 {
    fn default() -> Self {
        Self {
            用户名: Default::default(),
            计步数: Default::default(),
            踩点: Default::default(),
            地图: Some(1),
            缩放比例: 2.0,
            瓦片尺寸: 32.0,
        }
    }
}

pub struct 数据组件;

impl Plugin for 数据组件 {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_state(全局状态::加载中).init_resource::<全局数据>();
    }
}
