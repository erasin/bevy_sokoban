use crate::components::*;
use crate::data::*;
use bevy::prelude::*;

// 状态
#[derive(Clone)]
pub enum GameState {
    Loading, //加载中
    Menu,    // 菜单位置
    Playing, // 游戏中
    Over,    // 死亡
    Pause,   // 暂停
}

// // 组件
// pub struct GameStatePlugin;

// impl Plugin for GameStatePlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_system(handle_gamestate_system.system());
//     }
// }

// fn handle_gamestate_system(
//     mut game_data: ResMut<GameData>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut player_query: Query<(&Player, &mut Transform)>,
// ) {
// }
