use bevy::prelude::*;

use crate::score::resources::Score;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string())
    }
}
