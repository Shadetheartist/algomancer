use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase, CardPrototypeId};
use crate::game::state::card::Card;
use crate::game::state::card_collection::{CardCollectionId, FindCardCollectionMutResult};
use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutator;
use crate::game::state::permanent::Permanent;
use crate::game::state::region::RegionId;
use crate::game::state::State;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreatePermanentMutation {
    pub region_id: RegionId,
    pub permanent: Permanent
}

impl StateMutator for CreatePermanentMutation {
    fn mutate_state(&self, mut state: State, db: &CardPrototypeDatabase) -> Result<State, StateError> {

        state.next_permanent_id += 1;

        let region = state.find_region_mut(self.region_id)?;
        region.unformed_permanents.push(self.permanent.clone());

        Ok(state)
    }
}
