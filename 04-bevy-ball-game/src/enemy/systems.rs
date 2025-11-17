use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::*;
use rand::random;

use super::{ENEMY_NUMBERS, ENEMY_SPEED, ENEMY_SPRITE_SIZE};
use crate::enemy::components::*;
use crate::enemy::resources::*;

fn get_savety_spawn_coordinates(window: &Window, size: f32) -> Vec3 {
    let double_size = size * 2.0;

    let x_min = 0.0 + double_size;
    let x_max = window.width() - double_size;
    let y_min = 0.0 + double_size;
    let y_max = window.height() - double_size;

    let rand_x = random::<f32>() * window.width();
    let rand_y = random::<f32>() * window.height();

    let x = if rand_x < x_min {
        x_min
    } else if rand_x > x_max {
        x_max
    } else {
        rand_x
    };

    let y = if rand_y < y_min {
        y_min
    } else if rand_y > y_max {
        y_max
    } else {
        rand_y
    };

    Vec3 { x, y, z: 0.0 }
}

pub fn enemy_movement(mut eneny_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in eneny_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs()
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let window = window_query.single().unwrap();

    let half_size = ENEMY_SPRITE_SIZE / 2.0;
    let x_min = 0.0 + half_size;
    let x_max = window.width() - half_size;
    let y_min = 0.0 + half_size;
    let y_max = window.height() - half_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut is_direction_changed = false;
        let mut trans = transform.translation;

        // Bound player x position
        if trans.x < x_min {
            trans.x = x_min;
            enemy.direction = enemy.direction.reflect(Vec2::X);
            is_direction_changed = true;
        } else if trans.x > x_max {
            trans.x = x_max;
            enemy.direction = enemy.direction.reflect(Vec2::X);
            is_direction_changed = true;
        }

        // Bound player y position
        if trans.y < y_min {
            trans.y = y_min;
            enemy.direction = enemy.direction.reflect(Vec2::Y);
            is_direction_changed = true;
        } else if trans.y > y_max {
            trans.y = y_max;
            enemy.direction = enemy.direction.reflect(Vec2::Y);
            is_direction_changed = true;
        }

        // Нормализация направления после изменений
        enemy.direction = enemy.direction.normalize();

        if is_direction_changed {
            let sound1: Handle<bevy_kira_audio::AudioSource> =
                assets_server.load("audio/pluck_001.ogg");
            let sound2: Handle<bevy_kira_audio::AudioSource> =
                assets_server.load("audio/pluck_002.ogg");

            let sound = if random::<f32>() > 0.5 {
                sound1
            } else {
                sound2
            };
            audio.play(sound);
        }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    for _ in 0..ENEMY_NUMBERS {
        let direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();
        let translation = get_savety_spawn_coordinates(window, ENEMY_SPRITE_SIZE);

        commands.spawn((
            Enemy { direction },
            Transform::from_translation(translation),
            Sprite {
                image: assets_server.load("sprites/ball_red_large.png"),
                ..default()
            },
        ));
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timmer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timmer.timer.tick(time.delta());
}

pub fn enemy_spawn_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
    enemy_spawn_timmer: ResMut<EnemySpawnTimer>,
) {
    if enemy_spawn_timmer.timer.just_finished() {
        let window = window_query.single().unwrap();

        let direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();
        let translation = get_savety_spawn_coordinates(window, ENEMY_SPRITE_SIZE);

        commands.spawn((
            Enemy { direction },
            Transform::from_translation(translation),
            Sprite {
                image: assets_server.load("sprites/ball_red_large.png"),
                ..default()
            },
        ));
    }
}
