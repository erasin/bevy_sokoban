use crate::{事件模块::移动事件, 全局状态, 状态模块::标签};

use bevy::{
    app::AppExit,
    input::gamepad::{Gamepad, GamepadButton, GamepadEvent, GamepadEventType},
    prelude::*,
    utils::HashSet,
};

pub struct 控制插件;

impl Plugin for 控制插件 {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<手柄连接器>()
            .add_system_to_stage(CoreStage::PreUpdate, 手柄连接处理.system())
            .add_system_set(
                SystemSet::on_update(全局状态::游戏中)
                    // .with_run_criteria(FixedTimestep::step(0.55))
                    .with_system(键盘处理.system())
                    .with_system(手柄按键处理.system())
                    .label(标签::键盘处理),
            );

        app.add_system(全局键盘处理.system())
            .add_system(隐藏鼠标处理.system());
    }
}

fn 键盘处理(
    mut 移动事件发送器: EventWriter<移动事件>, 按键键值: ResMut<Input<KeyCode>>
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
        let mut x = 0;
        let mut y = 0;

        if 按键键值.just_released(KeyCode::W) || 按键键值.just_released(KeyCode::S) {
            if 按键键值.pressed(KeyCode::W) {
                y = 1;
            } else if 按键键值.pressed(KeyCode::S) {
                y = -1;
            } else {
                y = 0;
            }
        } else if 按键键值.just_pressed(KeyCode::W) {
            y = 1;
        } else if 按键键值.just_pressed(KeyCode::S) {
            y = -1;
        } else {
            // player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }

        if 按键键值.just_released(KeyCode::D) || 按键键值.just_released(KeyCode::A) {
            if 按键键值.pressed(KeyCode::D) {
                x = 1;
            } else if 按键键值.pressed(KeyCode::A) {
                x = -1;
            } else {
                x = 0;
            }
        } else if 按键键值.just_pressed(KeyCode::D) {
            x = 1;
        } else if 按键键值.just_pressed(KeyCode::A) {
            x = -1;
        } else {
            // player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        }

        if x != y {
            移动事件发送器.send(移动事件(x, y))
        }
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
    mut 移动事件发送器: EventWriter<移动事件>,
    lobby: Res<手柄连接器>,
    按钮: Res<Input<GamepadButton>>,
    按钮力度: Res<Axis<GamepadButton>>,
    摇杆: Res<Axis<GamepadAxis>>,
) {
    let mut x = 0;
    let mut y = 0;

    for gamepad in lobby.手柄.iter().cloned() {
        if 按钮.just_released(GamepadButton(gamepad, GamepadButtonType::South)) {
            y = -1;
        } else if 按钮.just_released(GamepadButton(gamepad, GamepadButtonType::North)) {
            y = 1;
        } else if 按钮.just_released(GamepadButton(gamepad, GamepadButtonType::East)) {
            x = 1;
        } else if 按钮.just_released(GamepadButton(gamepad, GamepadButtonType::West)) {
            x = -1;
        }

        if x != y {
            移动事件发送器.send(移动事件(x, y))
        }

        let right_trigger = 按钮力度
            .get(GamepadButton(gamepad, GamepadButtonType::RightTrigger2))
            .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = 摇杆
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
    }
}

fn 全局键盘处理(
    mut 按键键值: ResMut<Input<KeyCode>>,
    mut 当前状态: ResMut<State<全局状态>>,
    mut 系统退出发送器: EventWriter<AppExit>,
) {
    if 按键键值.just_released(KeyCode::Escape) {
        match *当前状态.current() {
            全局状态::加载中 => 系统退出发送器.send(AppExit),
            全局状态::主菜单 => 系统退出发送器.send(AppExit),
            _ => {
                当前状态.set(全局状态::主菜单).unwrap();
                按键键值.reset(KeyCode::Escape);
            }
        }
    };

    if 按键键值.just_released(KeyCode::Q) {
        系统退出发送器.send(AppExit)
    }
}

fn 隐藏鼠标处理(
    mut 窗口: ResMut<Windows>,
    鼠标按键: Res<Input<MouseButton>>,
    按键: Res<Input<KeyCode>>,
) {
    let 主窗口 = 窗口.get_primary_mut().unwrap();

    if 鼠标按键.just_pressed(MouseButton::Left) {
        主窗口.set_cursor_lock_mode(true);
        主窗口.set_cursor_visibility(false);
    }

    if 按键.just_pressed(KeyCode::Escape) {
        主窗口.set_cursor_lock_mode(false);
        主窗口.set_cursor_visibility(true);
    }
}
