use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub const STAR_NUMBERS: i32 = 10;
pub const STAR_SPRITE_SIZE: f32 = 30.0;

use resources::*;
use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StarSpawnTimer::default())
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, (spawn_star, tick_star_spawn_timer));
    }
}
