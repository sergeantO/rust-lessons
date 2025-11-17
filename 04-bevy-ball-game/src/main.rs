mod components;
mod events;
mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

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
