use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase};


use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutator};
use crate::game::state::region::RegionId;
use crate::game::state::State;




#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StackPassPriorityMutation {
    pub region_id: RegionId,
}

impl StateMutator for StackPassPriorityMutation {
    fn mutate_state(&self, mut state: State, _: &CardPrototypeDatabase) -> Result<State, StateError> {
        let region = state.find_region_mut(self.region_id)?;
        region.stack.pass_priority();
        Ok(state)
    }
}

#[macro_export]
macro_rules! stack_pass_priority {
    ($mutations:ident, $region_id:expr) => {
        $mutations.push($crate::game::state::mutation::StateMutation::Static(
            $crate::game::state::mutation::StaticStateMutation::StackPassPriority(
            crate::game::state::mutation::StackPassPriorityMutation {
                region_id: $region_id,
            },
        )));
    };
}
