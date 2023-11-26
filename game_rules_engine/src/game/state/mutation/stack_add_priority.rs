use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase};


use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutator};
use crate::game::state::player::PlayerId;
use crate::game::state::region::RegionId;
use crate::game::state::State;




#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StackAddPriorityMutation {
    pub region_id: RegionId,
    pub player_id: PlayerId
}

impl StateMutator for StackAddPriorityMutation {
    fn mutate_state(&self, mut state: State, _: &CardPrototypeDatabase) -> Result<State, StateError> {
        let region = state.find_region_mut(self.region_id)?;
        region.stack.push_priority(self.player_id);
        Ok(state)
    }
}

#[macro_export]
macro_rules! stack_add_priority {
    ($mutations:ident, $region_id:expr, $player_id:expr) => {
        $mutations.push($crate::game::state::mutation::StateMutation::Static(
            $crate::game::state::mutation::StaticStateMutation::StackAddPriority(
            $crate::game::state::mutation::StackAddPriorityMutation {
                region_id: $region_id,
                player_id: $player_id,
            },
        )));
    };
}