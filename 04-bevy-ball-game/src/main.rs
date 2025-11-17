use bevy::app::AppExit;
use bevy::input::keyboard::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioControl;
use bevy_kira_audio::AudioPlugin;
use rand::random;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SPRITE_SIZE: f32 = 64.0;

pub const STAR_NUMBERS: i32 = 10;
pub const STAR_SPRITE_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub const ENEMY_NUMBERS: i32 = 3;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SPRITE_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin)
        .insert_resource(Score::default())
        .insert_resource(StarSpawnTimer::default())
        .insert_resource(EnemySpawnTimer::default())
        .add_message::<GameOver>()
        .add_systems(Update, handle_game_over)
        .add_systems(Update, exit_game)
        .add_systems(
            Startup,
            (spawn_camera, spawn_player, spawn_stars, spawn_enemies),
        )
        .add_systems(Update, update_camera)
        .add_systems(Update, (player_movement, confine_player_movement))
        .add_systems(Update, (update_enemy_direction, enemy_movement))
        .add_systems(Update, enemy_hit_player)
        .add_systems(
            Update,
            (
                player_hit_star,
                update_score,
                spawn_star,
                tick_star_spawn_timer,
            ),
        )
        .add_systems(Update, (tick_enemy_spawn_timer, enemy_spawn_over_time))
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

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

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

#[derive(Message)]
pub struct GameOver {
    pub final_score: u32,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

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

fn update_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<PrimaryWindow>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().unwrap();

    if let Ok(mut transform) = camera_query.single_mut() {
        transform.translation.x = window.width() / 2.0;
        transform.translation.y = window.height() / 2.0;
    }
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

pub fn enemy_movement(mut eneny_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in eneny_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs()
    }
}

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

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string())
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

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::CapsLock) {
        app_exit_event_writer.write(AppExit::Success);
    }
}
