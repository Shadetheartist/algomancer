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
        
        count < 2
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, GameOptions};
    use crate::game::action::Action;
    use crate::game::state::GameMode;
    use crate::game::state::resource::Faction::{Fire, Wood};
    use crate::game::state::team_configuration::TeamConfiguration;

    // utility function to avoid code duplication
    // creates a pre-defined rng instance
    #[test]
    fn run_test_game() {
        let options = GameOptions {
            seed: [u8; 16],
            game_mode: GameMode::LiveDraft {
                selected_deck_types: vec![Wood, Fire],
                team_configuration: TeamConfiguration::Teams {
                    teams_of_players: vec![1, 1],
                },
            },
        };

        let mut game = Game::new(&options).unwrap();

        for _ in 0..100 {
            let actions = game.valid_actions();
            let mut actions_vec: Vec<Action> = actions.into_iter().collect();
            actions_vec.sort();

            let action = actions_vec.remove(0);
            let mutations = game.apply_action(action).unwrap();

        }
    }
}