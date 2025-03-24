use serde::{Deserialize, Serialize};
use crate::game_mode::GameMode;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Options {
    pub seed: u128,
    pub game_mode: GameMode
}

impl Default for Options {
    fn default() -> Self {
        Self { seed: 0xBEEB, game_mode: GameMode::default() }
    }
}
