use crate::全局状态;

use bevy::{
    input::gamepad::{Gamepad, GamepadButton, GamepadEvent, GamepadEventType},
    prelude::*,
    utils::HashSet,
};

pub struct 控制插件;

impl Plugin for 控制插件 {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<手柄连接器>()
            .init_resource::<动作集>()
            .add_system_to_stage(CoreStage::PreUpdate, 手柄连接处理.system())
            .add_system_set(
                SystemSet::on_update(全局状态::游戏中)
                    .with_system(键盘处理.system())
                    .label("action"),
            );
    }
}

#[derive(Default)]
pub struct 动作集 {
    pub 用户移动向量: Option<Vec2>,
}

fn 键盘处理(
    mut 动作: ResMut<动作集>,
    按键键值: Res<Input<KeyCode>>,
    mut 当前状态: ResMut<State<全局状态>>,
) {
    if 按键键值.just_released(KeyCode::W)
        || 按键键值.pressed(KeyCode::W)
        || 按键键值.just_released(KeyCode::A)
        || 按键键值.pressed(KeyCode::A)
        || 按键键值.just_released(KeyCode::S)
        || 按键键值.pressed(KeyCode::S)
        || 按键键值.just_released(KeyCode::D)
        || 按键键值.pressed(KeyCode::D)
    {
        let mut 移动值 = Vec2::ZERO;

        if 按键键值.just_released(KeyCode::W) || 按键键值.just_released(KeyCode::S) {
            if 按键键值.pressed(KeyCode::W) {
                移动值.y = 1.;
            } else if 按键键值.pressed(KeyCode::S) {
                移动值.y = -1.;
            } else {
                移动值.y = 0.;
            }
        } else if 按键键值.just_pressed(KeyCode::W) {
            移动值.y = 1.;
        } else if 按键键值.just_pressed(KeyCode::S) {
            移动值.y = -1.;
        } else {
            // player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }
        info!("W ->>");

        if 按键键值.just_released(KeyCode::D) || 按键键值.just_released(KeyCode::A) {
            if 按键键值.pressed(KeyCode::D) {
                移动值.x = 1.;
            } else if 按键键值.pressed(KeyCode::A) {
                移动值.x = -1.;
            } else {
                移动值.x = 0.;
            }
        } else if 按键键值.just_pressed(KeyCode::D) {
            移动值.x = 1.;
        } else if 按键键值.just_pressed(KeyCode::A) {
            移动值.x = -1.;
        } else {
            // player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        }

        info!("{}", 移动值);

        if 移动值 != Vec2::ZERO {
            移动值 = 移动值.normalize();
            动作.用户移动向量 = Some(移动值);
        }
    } else {
        动作.用户移动向量 = None
    }

    if 按键键值.just_released(KeyCode::Q) {
        当前状态.set(全局状态::菜单).unwrap();
    }
}

#[derive(Default)]
struct 手柄连接器 {
    手柄: HashSet<Gamepad>,
}

fn 手柄连接处理(
    mut lobby: ResMut<手柄连接器>,
    mut 手柄事件读取器: EventReader<GamepadEvent>,
) {
    for event in 手柄事件读取器.iter() {
        match &event {
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                lobby.手柄.insert(*gamepad);
                info!("{:?} Connected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                lobby.手柄.remove(gamepad);
                info!("{:?} Disconnected", gamepad);
            }
            _ => (),
        }
    }
}

fn 手柄按键处理(
    mut actions: ResMut<动作集>,
    lobby: Res<手柄连接器>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    let mut 移动值 = Vec2::ZERO;

    for gamepad in lobby.手柄.iter().cloned() {
        if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::South)) {
            移动值.y = -1.;
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::North)) {
            移动值.y = 1.;
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::East)) {
            移动值.x = 1.;
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::West)) {
            移动值.x = -1.;
        } else {
            移动值.y = actions.用户移动向量.unwrap_or(Vec2::ZERO).y;
        }

        if 移动值 != Vec2::ZERO {
            移动值 = 移动值.normalize();
            actions.用户移动向量 = Some(移动值);
        }

        let right_trigger = button_axes
            .get(GamepadButton(gamepad, GamepadButtonType::RightTrigger2))
            .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
    }
}
