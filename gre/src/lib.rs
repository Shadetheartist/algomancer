mod action;
mod card;
mod database;
mod history;
mod object;
mod options;
mod player;
mod rng;
mod state;
mod zone;
mod library;
mod priority;
mod event;
mod permanent;
mod ability;
mod effect;
mod resource_type;
mod faction;
mod cost;
mod affinity;
mod card_type;
mod timing;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::action::{ActionError};
use crate::database::Database;
use crate::state::StateError;
use history::HistoryItem;
use state::State;

pub use crate::options::Options;
use action::Action;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRulesEngine {
    state: State,
    database: Database,
    history: Vec<HistoryItem>,
}

impl GameRulesEngine {
    fn push_history_item(&mut self, action: &Action) {
        self.history
            .push(HistoryItem::from_state_action_pair(&self.state, action))
    }

    pub fn apply_action(&mut self, action: &Action) -> Result<(), GameRulesEngineError> {
        let next_state = self.state.apply_action(action, &self.database)?;
        self.push_history_item(action);
        self.state = next_state;
        Ok(())
    }

    pub fn valid_actions(&self) -> Vec<Action> {
        self.state.valid_actions()
    }
}

#[derive(Error, Debug)]
pub enum GameRulesEngineError {
    #[error("state error: {0}")]
    StateError(#[from] StateError),
    #[error("action error: {0}")]
    ActionError(#[from] ActionError),
}

impl TryFrom<&Options> for GameRulesEngine {
    type Error = GameRulesEngineError;

    fn try_from(options: &Options) -> Result<Self, Self::Error> {
        let gre = Self {
            state: State::try_from(options)?,
            database: Default::default(),
            history: Default::default(),
        };

        Ok(gre)
    }
}

#[cfg(test)]
mod tests {
    use crate::options::Options;
    use crate::GameRulesEngine;

    #[test]
    fn test() {
        let gre = GameRulesEngine::try_from(&Options::default()).unwrap();
    }
}
