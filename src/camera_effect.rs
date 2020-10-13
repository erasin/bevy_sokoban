use bevy::prelude::*;
use bevy::render::camera::Camera;
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
        app.add_resource(CameraData {
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
        .spawn(Camera2dComponents::default())
        .with(CameraTarget);
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

        for (_, mut trans) in &mut query.iter() {
            // if camera.name.eq(&Some("Camera2d".to_string())) {
            // 相机 z 高度位置需要高于要显示的对象
            let x = trans.translation().x();
            let y = trans.translation().y();
            let z = trans.translation().z();
            trans.set_translation(Vec3::new(x + x_target, y + y_target, z))
            // }
        }
    }
    camera_data.timer.tick(time.delta_seconds);
    if camera_data.timer.finished {
        camera_data.state = CameraState::Normal;

        for (_, mut trans) in &mut query.iter() {
            let z = trans.translation().z();
            trans.set_translation(Vec3::new(0.0, 0.0, z));
        }
    }
}

use crate::components::Player;

// 中心跟随
// 矩形跟随
fn camera_follow_system(
    camera_data: Res<CameraData>,
    mut player_query: Query<(&Player, &Transform)>,
    mut camera_query: Query<(&CameraTarget, &mut Transform)>,
) {
    if camera_data.state == CameraState::Follow {
        for (_, player_trans) in &mut player_query.iter() {
            for (_, mut camera_trans) in &mut camera_query.iter() {
                let x = player_trans.translation().x();
                let y = player_trans.translation().y();
                let z = camera_trans.translation().z();
                camera_trans.set_translation(Vec3::new(x, y, z));
            }
        }
    }
}
