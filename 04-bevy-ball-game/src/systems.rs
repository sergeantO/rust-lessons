use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::*;

use crate::events::*;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d {},
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 1.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: bevy::camera::ScalingMode::WindowSize,
            area: Rect {
                min: Vec2 { x: 0.0, y: 0.0 },
                max: Vec2 {
                    x: window.width(),
                    y: window.height(),
                },
            },
        }),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn update_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<PrimaryWindow>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().unwrap();

    if let Ok(mut transform) = camera_query.single_mut() {
        transform.translation.x = window.width() / 2.0;
        transform.translation.y = window.height() / 2.0;
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::CapsLock) {
        app_exit_event_writer.write(AppExit::Success);
    }
}

pub fn handle_game_over(
    mut game_over_message: MessageReader<GameOver>,
    assets_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let iter = game_over_message.read();
    for message in iter {
        let destroy_sound: Handle<bevy_kira_audio::AudioSource> =
            assets_server.load("audio/explosionCrunch_000.ogg");
        audio.play(destroy_sound);
        println!(
            "Game Over. Final score is {}",
            message.final_score.to_string()
        );
    }
}
