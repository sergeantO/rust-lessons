mod game;
mod main_menu;
mod systems;

use game::*;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin))
        .insert_state(AppState::MainMenu)
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (transition_to_game_state, transition_to_main_menu_state),
        )
        .add_systems(Update, update_camera)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, exit_game)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Game,
    GameOver,
}
