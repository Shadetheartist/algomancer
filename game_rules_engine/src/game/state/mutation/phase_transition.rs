use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase};


use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutator};
use crate::game::state::progression::Phase;
use crate::game::state::region::RegionId;
use crate::game::state::State;




#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PhaseTransitionMutation {
    pub region_id: RegionId,
    pub to_phase: Phase,
}

impl StateMutator for PhaseTransitionMutation {
    fn mutate_state(&self, mut state: State, _: &CardPrototypeDatabase) -> Result<State, StateError> {
        state = state.region_transition_to_next_step(self.region_id);
        Ok(state)
    }
}

#[macro_export]
macro_rules! phase_transition {
    ($mutations:ident, $region_id:expr, $phase:expr) => {
        $mutations.push($crate::game::state::mutation::StateMutation::Static(
            $crate::game::state::mutation::StaticStateMutation::PhaseTransition(
            crate::game::state::mutation::PhaseTransitionMutation {
                region_id: $region_id,
                to_phase: $phase
            },
        )));
    };
}
