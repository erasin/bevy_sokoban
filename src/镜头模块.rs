use crate::{状态模块::全局状态, 组件模块::玩家};
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::time::Duration;

pub struct 镜头;

#[derive(PartialEq)]
pub enum 镜头状态 {
    正常,
    抖动,
    跟随, // 用户跟随
}

pub struct 镜头数据 {
    pub 状态: 镜头状态,
    pub 计时器: Timer,
}

pub struct 镜头特效插件 {
    抖动持续时长: Duration,
}

impl 镜头特效插件 {
    pub fn new(seconds: f32) -> Self {
        镜头特效插件 {
            抖动持续时长: Duration::from_secs_f32(seconds),
        }
    }
}

impl Plugin for 镜头特效插件 {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(镜头数据 {
            状态: 镜头状态::正常,
            计时器: Timer::new(self.抖动持续时长, true),
        })
        .add_startup_system(初始化处理.system())
        .add_system_set(
            SystemSet::on_update(全局状态::游戏中)
                .with_system(镜头抖动处理.system())
                .with_system(镜头跟随处理.system()),
        );
    }
}

fn 初始化处理(mut 指令: Commands) {
    指令
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(镜头);
}

// 抖动原型
fn 镜头抖动处理(
    系统时间: Res<Time>,
    mut 当前镜头: ResMut<镜头数据>,
    mut query: Query<(&镜头, &mut Transform)>,
) {
    if 当前镜头.状态 == 镜头状态::抖动 {
        let mut rng = thread_rng();
        let x_target: f32 = rng.gen_range(-20.0..20.0);
        let y_target: f32 = rng.gen_range(-20.0..20.0);

        for (_, mut trans) in query.iter_mut() {
            // if camera.name.eq(&Some("Camera2d".to_string())) {
            // 相机 z 高度位置需要高于要显示的对象
            trans.translation.x += x_target;
            trans.translation.y += y_target;
        }
    }

    if 当前镜头.计时器.tick(系统时间.delta()).finished() {
        当前镜头.状态 = 镜头状态::正常;

        for (_, mut trans) in query.iter_mut() {
            trans.translation.x = 0.0;
            trans.translation.y = 0.0;
        }
    }
}

// 中心跟随
// 矩形跟随
fn 镜头跟随处理(
    系统时间: Res<Time>,
    当前镜头: Res<镜头数据>,
    mut query: QuerySet<(
        Query<&Transform, With<玩家>>,
        Query<&mut Transform, With<镜头>>,
    )>,
) {
    if 当前镜头.状态 == 镜头状态::跟随 {
        let delta = 系统时间.delta_seconds();

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
