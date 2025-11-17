use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use crate::game::star::{components::Star, resources::StarSpawnTimer};

use super::STAR_NUMBERS;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    for _ in 0..STAR_NUMBERS {
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands.spawn((
            Star {},
            Transform::from_xyz(rand_x, rand_y, 0.0),
            Sprite {
                image: assets_server.load("sprites/star.png"),
                ..default()
            },
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_star(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.just_finished() {
        let window = window_query.single().unwrap();
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.width();

        commands.spawn((
            Star {},
            Transform::from_xyz(rand_x, rand_y, 0.0),
            Sprite {
                image: assets_server.load("sprites/star.png"),
                ..default()
            },
        ));
    }
}
