use bevy::prelude::*;

#[derive(Message)]
pub struct GameOver {
    pub final_score: u32,
}
