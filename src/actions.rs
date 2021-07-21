use crate::GameState;

use bevy::{
    input::gamepad::{Gamepad, GamepadButton, GamepadEvent, GamepadEventType},
    prelude::*,
    utils::HashSet,
};

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GamepadLobby>()
            .init_resource::<Actions>()
            .add_system_to_stage(CoreStage::PreUpdate, gamepad_connection_system.system())
            // .add_system_set_to_stage(
            //     CoreStage::PreUpdate,
            //     SystemSet::on_update(GameState::Playing)
            //         .with_system(gamepad_connection_system.system()),
            // )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(set_movement_actions.system())
                    // .with_system(gamepad_system.system())
                    .label("action"),
            );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if keyboard_input.just_released(KeyCode::W)
        || keyboard_input.pressed(KeyCode::W)
        || keyboard_input.just_released(KeyCode::A)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.just_released(KeyCode::S)
        || keyboard_input.pressed(KeyCode::S)
        || keyboard_input.just_released(KeyCode::D)
        || keyboard_input.pressed(KeyCode::D)
    {
        let mut player_movement = Vec2::ZERO;

        if keyboard_input.just_released(KeyCode::W) || keyboard_input.just_released(KeyCode::S) {
            if keyboard_input.pressed(KeyCode::W) {
                player_movement.y = 1.;
            } else if keyboard_input.pressed(KeyCode::S) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if keyboard_input.just_pressed(KeyCode::W) {
            player_movement.y = 1.;
        } else if keyboard_input.just_pressed(KeyCode::S) {
            player_movement.y = -1.;
        } else {
            // player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }
        info!("W ->>");

        if keyboard_input.just_released(KeyCode::D) || keyboard_input.just_released(KeyCode::A) {
            if keyboard_input.pressed(KeyCode::D) {
                player_movement.x = 1.;
            } else if keyboard_input.pressed(KeyCode::A) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if keyboard_input.just_pressed(KeyCode::D) {
            player_movement.x = 1.;
        } else if keyboard_input.just_pressed(KeyCode::A) {
            player_movement.x = -1.;
        } else {
            // player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        }

        info!("{}", player_movement);

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            actions.player_movement = Some(player_movement);
        }
    } else {
        // actions.player_movement = None;
    }

    if keyboard_input.just_released(KeyCode::Q) {
        state.set(GameState::Menu).unwrap();
    }
}

#[derive(Default)]
struct GamepadLobby {
    gamepads: HashSet<Gamepad>,
}

fn gamepad_connection_system(
    mut lobby: ResMut<GamepadLobby>,
    mut gamepad_event: EventReader<GamepadEvent>,
) {
    for event in gamepad_event.iter() {
        match &event {
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                lobby.gamepads.insert(*gamepad);
                info!("{:?} Connected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                lobby.gamepads.remove(gamepad);
                info!("{:?} Disconnected", gamepad);
            }
            _ => (),
        }
    }
}

fn gamepad_system(
    mut actions: ResMut<Actions>,
    lobby: Res<GamepadLobby>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    let mut player_movement = Vec2::ZERO;

    for gamepad in lobby.gamepads.iter().cloned() {
        if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::South)) {
            player_movement.y = -1.;
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::North)) {
            player_movement.y = 1.;
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::East)) {
            player_movement.x = 1.;
        } else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::West)) {
            player_movement.x = -1.;
        } else {
            player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            actions.player_movement = Some(player_movement);
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
