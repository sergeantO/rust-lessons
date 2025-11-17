pub mod enemy;
pub mod events;
pub mod score;
mod systems;

mod player;
mod star;

use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::{app::AppExtStates, condition::in_state, state::States},
};
use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<SimulationState>()
            .add_message::<GameOver>()
            .add_plugins((StarPlugin, PlayerPlugin, EnemyPlugin, ScorePlugin))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
}
