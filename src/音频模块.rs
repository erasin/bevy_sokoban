use crate::全局状态;
use crate::加载模块::AudioAssets;
use crate::行为模块::动作集;

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

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.set_volume_in_channel(0.3, &channels.flying);
    audio.play_looped_in_channel(audio_assets.audio_wall.clone(), &channels.flying);
    audio.pause_channel(&channels.flying);
}

fn stop_audio(audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.stop_channel(&channels.flying);
}

fn control_flying_sound(actions: Res<动作集>, audio: Res<Audio>, channels: Res<AudioChannels>) {
    if actions.用户移动向量.is_some() {
        audio.resume_channel(&channels.flying);
    } else {
        audio.pause_channel(&channels.flying)
    }
}
