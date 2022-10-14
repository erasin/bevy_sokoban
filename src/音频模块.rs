use crate::加载模块::音频素材;
use crate::{事件模块::移动事件, 全局状态};

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::Audio;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(全局状态::游戏中).with_system(start_audio))
            .add_system_set(
                SystemSet::on_update(全局状态::游戏中).with_system(control_flying_sound),
            )
            .add_system_set(SystemSet::on_exit(全局状态::游戏中).with_system(stop_audio));
    }
}

// struct MusicController(Handle<AudioSink>);
struct MusicController(Handle<AudioInstance>);

fn start_audio(
    mut commands: Commands,
    音频资源: Res<音频素材>,
    audio: Res<Audio>,
    // audio_sinks: Res<Assets<AudioSink>>,
) {
    // bevy audio
    // let handle = audio_sinks.get_handle(audio.play(音频资源.wall.as_weak()));

    // kira
    audio.pause();
    let handle = audio.play(音频资源.wall.clone()).with_volume(0.3).handle();

    commands.insert_resource(MusicController(handle));
}

fn stop_audio(mut audio_instance: ResMut<Assets<AudioInstance>>, music: Res<MusicController>) {
    if let Some(sink) = audio_instance.get_mut(&music.0) {
        match sink.state() {
            PlaybackState::Stopped => {}
            PlaybackState::Stopping { .. } => {}
            _ => {
                sink.stop(AudioTween::default());
            }
        }
    }
}

fn control_flying_sound(
    mut 移动事件接收器: EventReader<移动事件>,
    mut audio_sinks: ResMut<Assets<AudioInstance>>,
    music: Res<MusicController>,
) {
    let sink = audio_sinks.get_mut(&music.0).unwrap();
    for ev in 移动事件接收器.iter() {
        if ev.0 != ev.1 {
            sink.resume(AudioTween::default());
        } else {
            sink.pause(AudioTween::default());
        }
    }
}
