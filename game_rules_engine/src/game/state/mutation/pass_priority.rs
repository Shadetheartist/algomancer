use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase};


use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutator;
use crate::game::state::player::PlayerId;
use crate::game::state::stack::Next;
use crate::game::state::State;




#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SetPlayerPassedPriorityMutation {
    pub player_id: PlayerId,
    pub value: bool
}

impl StateMutator for SetPlayerPassedPriorityMutation {
    fn mutate_state(&self, mut state: State, _: &CardPrototypeDatabase) -> Result<State, StateError> {
        let region = state.find_region_containing_player_mut(self.player_id);
        match region.stack.next() {
            Next::PassPriority(_) => {
                region.stack.pass_priority();
            }
            Next::TransitionStep => { panic!(); }
            Next::ResolveEffect(_) => { panic!(); }
        }
        Ok(state)
    }
}
