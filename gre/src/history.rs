use std::hash::{DefaultHasher, Hash, Hasher};
use serde::{Deserialize, Serialize};
use crate::Action;
use crate::state::State;

#[derive(Serialize, Deserialize)]
pub struct HistoryItem {
    action: Action,
    state_hash: u64
}

impl HistoryItem {
    pub fn from_state_action_pair(state: &State, action: &Action) -> Self {
        let mut hasher = DefaultHasher::default();
        state.hash(&mut hasher);
        let state_hash = hasher.finish();

        Self { action: action.clone(), state_hash: state_hash }
    }
}