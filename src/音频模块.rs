use crate::加载模块::音频素材;
use crate::{事件模块::移动事件, 全局状态};

use bevy::audio::AudioSink;
use bevy::prelude::*;
// use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(AudioChannels {
        //     flying: AudioChannel::new("flying".to_owned()),
        // })
        // .add_plugin(AudioPlugin)
        app.add_system_set(SystemSet::on_enter(全局状态::游戏中).with_system(start_audio))
            .add_system_set(
                SystemSet::on_update(全局状态::游戏中).with_system(control_flying_sound),
            )
            .add_system_set(SystemSet::on_exit(全局状态::游戏中).with_system(stop_audio));
    }
}

struct MusicController(Handle<AudioSink>);

fn start_audio(
    mut commands: Commands,
    音频资源: Res<音频素材>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let handle = audio_sinks.get_handle(audio.play(音频资源.audio_wall.as_weak()));
    commands.insert_resource(MusicController(handle));
}

fn stop_audio(mut audio_sinks: ResMut<Assets<AudioSink>>, music: Res<MusicController>) {
    if let Some(sink) = audio_sinks.get(&music.0) {
        sink.stop();
    }
}

fn control_flying_sound(
    mut 移动事件接收器: EventReader<移动事件>,
    mut audio_sinks: ResMut<Assets<AudioSink>>,
    music: Res<MusicController>,
) {
    let sink = audio_sinks.get(&music.0).unwrap();
    for ev in 移动事件接收器.iter() {
        if ev.0 != ev.1 {
            sink.play()
        } else {
            sink.pause()
        }
    }
}
