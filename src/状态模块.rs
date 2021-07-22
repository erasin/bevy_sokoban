#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum 全局状态 {
    加载中,
    菜单,
    游戏中,
    结束,
    暂停,
}
