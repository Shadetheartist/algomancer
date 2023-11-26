use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase};


use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutator};
use crate::game::state::region::RegionId;
use crate::game::state::State;




#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StackClearPriorityMutation {
    pub region_id: RegionId,
}

impl StateMutator for StackClearPriorityMutation {
    fn mutate_state(&self, mut state: State, _: &CardPrototypeDatabase) -> Result<State, StateError> {
        let region = state.find_region_mut(self.region_id)?;
        region.stack.clear_priority();
        Ok(state)
    }
}

#[macro_export]
macro_rules! stack_clear_priority {
    ($mutations:ident, $region_id:expr) => {
        $mutations.push($crate::game::state::mutation::StateMutation::Static(
            $crate::game::state::mutation::StaticStateMutation::StackClearPriority(
            $crate::game::state::mutation::StackClearPriorityMutation {
                region_id: $region_id,
            },
        )));
    };
}
