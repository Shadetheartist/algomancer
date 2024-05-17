mod rng;
mod options;
mod state;
mod history;
mod action;

use state::State;
use history::HistoryItem;
use action::Action;
use crate::options::Options;

pub struct GameRulesEngine {
    state: State,
    database: Database,
    history: Vec<HistoryItem>
}

pub struct Database {}

impl GameRulesEngine {
    pub fn new(options: Options) -> Self {
        Self {
            state: State::new(options),
            database: Database {},
            history: vec![],
        }
    }

    fn push_history_item(&mut self, action: &Action) {
        self.history.push(HistoryItem::new(&self.state, action))
    }

    fn apply_action() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_history(){

    }
}