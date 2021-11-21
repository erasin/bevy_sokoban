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
use debug::调试组件;
use 加载模块::加载素材库插件;
use 地图模块::地图插件;
use 数据模块::*;
use 状态模块::*;
use 界面模块::*;
use 系统模块::*;
use 菜单模块::主菜单组件;
use 行为模块::控制插件;

use 镜头模块::镜头特效插件;

pub struct 组件集合;

impl PluginGroup for 组件集合 {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(数据组件)
            .add(加载素材库插件)
            .add(主菜单组件)
            .add(地图插件)
            .add(镜头特效插件::new(0.5))
            .add(界面组件)
            .add(控制插件)
            .add(行为组件)
            .add(调试组件);
    }
}
