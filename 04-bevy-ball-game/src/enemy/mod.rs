use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub const ENEMY_NUMBERS: i32 = 3;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SPRITE_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer::default())
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, (update_enemy_direction, enemy_movement))
            .add_systems(Update, (tick_enemy_spawn_timer, enemy_spawn_over_time));
    }
}
