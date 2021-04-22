/// 状态
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading, //加载中
    Menu,    // 菜单位置
    Playing, // 游戏中
    Over,    // 死亡
    Pause,   // 暂停
}
