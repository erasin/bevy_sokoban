use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::time::Duration;

pub struct CameraTarget;

#[derive(PartialEq)]
pub enum CameraState {
    Normal,
    Shake,
    Follow, // 用户跟随
}

pub struct CameraData {
    pub state: CameraState,
    pub timer: Timer,
}

pub struct CameraEffectPlugin {
    shake_duration: Duration,
}

impl CameraEffectPlugin {
    pub fn new(seconds: f32) -> Self {
        CameraEffectPlugin {
            shake_duration: Duration::from_secs_f32(seconds),
        }
    }
}

impl Plugin for CameraEffectPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(CameraData {
            state: CameraState::Normal,
            timer: Timer::new(self.shake_duration, true),
        })
        .add_startup_system(setup_system.system())
        .add_system(camera_shake_system.system())
        .add_system(camera_follow_system.system());
    }
}

fn setup_system(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraTarget);
}

// 抖动原型
fn camera_shake_system(
    time: Res<Time>,
    mut camera_data: ResMut<CameraData>,
    mut query: Query<(&CameraTarget, &mut Transform)>,
) {
    if camera_data.state == CameraState::Shake {
        let mut rng = thread_rng();
        let x_target: f32 = rng.gen_range(-20.0, 20.0);
        let y_target: f32 = rng.gen_range(-20.0, 20.0);

        for (_, mut trans) in query.iter_mut() {
            // if camera.name.eq(&Some("Camera2d".to_string())) {
            // 相机 z 高度位置需要高于要显示的对象
            trans.translation.x += x_target;
            trans.translation.y += y_target;
        }
    }
    camera_data.timer.tick(time.delta());
    if camera_data.timer.finished() {
        camera_data.state = CameraState::Normal;

        for (_, mut trans) in query.iter_mut() {
            trans.translation.x = 0.0;
            trans.translation.y = 0.0;
        }
    }
}

use crate::components::Player;

// 中心跟随
// 矩形跟随
fn camera_follow_system(
    time: Res<Time>,
    camera_data: Res<CameraData>,
    mut query: QuerySet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<CameraTarget>>,
    )>,
) {
    if camera_data.state == CameraState::Follow {
        let delta = time.delta_seconds();

        let mut x = 0.0;
        let mut y = 0.0;

        for player_trans in query.q0().iter() {
            x = player_trans.translation.x;
            y = player_trans.translation.y;
        }

        for mut camera_trans in query.q1_mut().iter_mut() {
            camera_trans.translation.x = x * delta;
            camera_trans.translation.y = y * delta;
        }
    }
}
