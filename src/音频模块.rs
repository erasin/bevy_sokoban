use crate::加载模块::音频素材;
use crate::{事件模块::移动事件, 全局状态};

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AudioChannels {
            flying: AudioChannel::new("flying".to_owned()),
        })
        .add_plugin(AudioPlugin)
        .add_system_set(SystemSet::on_enter(全局状态::游戏中).with_system(start_audio.system()))
        .add_system_set(
            SystemSet::on_update(全局状态::游戏中).with_system(control_flying_sound.system()),
        )
        .add_system_set(SystemSet::on_exit(全局状态::游戏中).with_system(stop_audio.system()));
    }
}

struct AudioChannels {
    flying: AudioChannel,
}

fn start_audio(
    音频资源: Res<音频素材>, 音频: Res<Audio>, 音频信道: Res<AudioChannels>
) {
    音频.set_volume_in_channel(0.3, &音频信道.flying);
    音频.play_looped_in_channel(音频资源.audio_wall.clone(), &音频信道.flying);
    音频.pause_channel(&音频信道.flying);
}

fn stop_audio(音频: Res<Audio>, 音频信道: Res<AudioChannels>) {
    音频.stop_channel(&音频信道.flying);
}

fn control_flying_sound(
    mut 移动事件接收器: EventReader<移动事件>,
    音频: Res<Audio>,
    音频信道: Res<AudioChannels>,
) {
    for ev in 移动事件接收器.iter() {
        if ev.0 != ev.1 {
            音频.resume_channel(&音频信道.flying);
        } else {
            音频.pause_channel(&音频信道.flying)
        }
    }
}
