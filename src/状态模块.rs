use bevy::prelude::SystemLabel;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum 全局状态 {
    加载中,
    主菜单,
    // 关卡菜单,
    游戏中,
    // 结束,
    // 暂停,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum 标签 {
    地图加载,
    键盘处理,
}
