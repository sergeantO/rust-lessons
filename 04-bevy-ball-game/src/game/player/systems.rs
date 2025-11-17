use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::*;

use super::components::Player;
use crate::enemy::ENEMY_SPRITE_SIZE;
use crate::enemy::components::Enemy;
use crate::game::star::components::Star;
use crate::score::resources::Score;
use crate::{events::GameOver, game::star::STAR_SPRITE_SIZE};

use super::{PLAYER_SPEED, PLAYER_SPRITE_SIZE};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Player {},
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Sprite {
            image: assets_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_transform_query.single_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction != Vec2::ZERO {
            let delta = direction.normalize() * PLAYER_SPEED * time.delta_secs();
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
        }
    }
}

pub fn confine_player_movement(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_transform_query.single_mut() {
        let window = window_query.single().unwrap();

        let half_size = PLAYER_SPRITE_SIZE / 2.0;
        let x_min = 0.0 + half_size;
        let x_max = window.width() - half_size;
        let y_min = 0.0 + half_size;
        let y_max = window.height() - half_size;

        let mut translation = player_transform.translation;

        // Bound player x position
        if translation.x < x_min {
            translation.x = x_min
        } else if translation.x > x_max {
            translation.x = x_max
        }

        // Bound player y position
        if translation.y < y_min {
            translation.y = y_min
        } else if translation.y > y_max {
            translation.y = y_max
        }

        player_transform.translation = translation
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_message: MessageWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,

    score: Res<Score>,
) {
    let player_radius = PLAYER_SPRITE_SIZE / 2.0;
    let enemy_radius = ENEMY_SPRITE_SIZE / 2.0;
    let min_distance = player_radius + enemy_radius;

    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        let player_translation = player_transform.translation;

        for enemy_transform in enemy_query.iter() {
            let distance = enemy_transform.translation.distance(player_translation);

            if distance < min_distance {
                commands.entity(player_entity).despawn();
                game_over_message.write(GameOver {
                    final_score: score.value,
                });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    assets_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (strar_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance <= PLAYER_SPRITE_SIZE / 2.0 + STAR_SPRITE_SIZE / 2.0 {
                score.value += 1;
                commands.entity(strar_entity).despawn();
                println!("Player hit star");

                let sound_effect: Handle<bevy_kira_audio::AudioSource> =
                    assets_server.load("audio/impactGlass_heavy_001.ogg");
                audio.play(sound_effect);
            }
        }
    }
}
