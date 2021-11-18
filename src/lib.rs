mod debug;
#[path = "事件模块.rs"]
mod 事件模块;
#[path = "加载模块.rs"]
mod 加载模块;
#[path = "地图模块.rs"]
mod 地图模块;
#[path = "数据模块.rs"]
mod 数据模块;
#[path = "状态模块.rs"]
mod 状态模块;
#[path = "界面模块.rs"]
mod 界面模块;
#[path = "系统模块.rs"]
mod 系统模块;
#[path = "组件模块.rs"]
mod 组件模块;
#[path = "菜单模块.rs"]
mod 菜单模块;
#[path = "行为模块.rs"]
mod 行为模块;
#[path = "镜头模块.rs"]
mod 镜头模块;
#[path = "音频模块.rs"]
mod 音频模块;

use bevy::prelude::*;
use debug::DebugPlugin;
use 加载模块::加载素材库插件;
use 地图模块::地图插件;
use 数据模块::*;
use 状态模块::*;
use 界面模块::*;
use 系统模块::*;
use 菜单模块::菜单组件;
use 行为模块::控制插件;

use 镜头模块::镜头特效插件;

/// 瓦片宽度大小设定
const 瓦片尺寸: f32 = 32.0;
/// 瓦片放大比例
const 缩放比例: f32 = 2.0;

pub struct 组件集合;

impl Plugin for 组件集合 {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(全局状态::加载中)
            .add_plugin(加载素材库插件)
            .add_plugin(菜单组件)
            // .add_plugin(网格插件)
            .add_plugin(地图插件)
            .add_plugin(镜头特效插件::new(0.5))
            .add_plugin(UIPlugin)
            .add_plugin(控制插件)
            .add_plugin(行为组件)
            .init_resource::<全局数据>()
            .add_plugin(DebugPlugin);
    }
}
