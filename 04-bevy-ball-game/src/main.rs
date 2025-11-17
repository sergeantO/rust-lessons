mod events;
mod systems;

pub mod enemy;
pub mod score;

mod player;
mod star;

use events::*;
use systems::*;

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

use crate::{enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin, star::StarPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin))
        .add_plugins((StarPlugin, PlayerPlugin, EnemyPlugin, ScorePlugin))
        .add_message::<GameOver>()
        .add_systems(Update, handle_game_over)
        .add_systems(Update, exit_game)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, update_camera)
        .run();
}
