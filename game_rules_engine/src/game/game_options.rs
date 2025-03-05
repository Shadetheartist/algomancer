use crate::game::state::team_configuration::TeamConfiguration;
use crate::game::state::GameMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameOptions {
    pub seed: [u8; 16],
    pub game_mode: GameMode,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            seed: [0; 16],
            game_mode: GameMode::LiveDraft {
                selected_deck_types: vec![],
                team_configuration: TeamConfiguration::Ffa { num_players: 4 },
            },
        }
    }
}
