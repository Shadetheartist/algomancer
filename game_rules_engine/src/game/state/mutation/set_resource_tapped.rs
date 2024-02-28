use serde::{Deserialize, Serialize};
use database::{CardPrototypeDatabase};
use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutator;
use crate::game::state::permanent::{Permanent, PermanentId};
use crate::game::state::State;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SetResourceTappedMutation {
    pub resource_permanent_id: PermanentId,
    pub tapped: bool,
}

impl StateMutator for SetResourceTappedMutation {
    fn mutate_state(&self, mut state: State, _db: &CardPrototypeDatabase) -> Result<State, StateError> {
        let id = self.resource_permanent_id;
        let permanent = state.find_permanent_mut(id)?;
        if let Permanent::Resource { tapped, .. } = permanent {
            *tapped = self.tapped;
        } else {
            panic!("tap resource mutation used on a non-resource permanent")
        }
        Ok(state)
    }
}
