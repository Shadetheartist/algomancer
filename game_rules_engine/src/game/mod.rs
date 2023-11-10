use serde::{Deserialize, Serialize};

use state::rng::AlgomancerRngSeed;

use crate::game::action::Action;
use crate::game::state::card::CardPrototypeDatabase;
use crate::game::state::GameMode;

pub mod state;
pub mod action;
pub mod game_builder;

#[derive(Debug)]
pub struct GameOptions {
    pub seed: AlgomancerRngSeed,
    pub game_mode: GameMode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub state: state::State,
    pub action_history: Vec<Action>,
    pub cards_db: CardPrototypeDatabase,
}

impl Game {

    // is_over returns true if there are are any living players on at least two teams
    pub fn is_over(&self) -> bool {
        let filtered = self.state.team_ids().into_iter().filter(|&t| !self.state.living_players_in_team(t).is_empty());
        let count = filtered.take(2).count();
        let result = count < 2;
        result
    }

}