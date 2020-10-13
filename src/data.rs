#[derive(Default)]
pub struct GameData {
    pub name: String,     // 用户数
    pub step: i32,        // 步数
    pub spot: i32,        // 踩点
    pub map: Option<i32>, // 地图
}
