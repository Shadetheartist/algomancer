use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase};


use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutator};
use crate::game::state::region::RegionId;
use crate::game::state::State;




#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct PhaseTransitionMutation {
    pub region_id: RegionId,
}

impl StateMutator for PhaseTransitionMutation {
    fn mutate_state(&self, mut state: State, _: &CardPrototypeDatabase) -> Result<State, StateError> {
        state = state.region_transition_to_next_step(self.region_id);
        Ok(state)
    }
}

