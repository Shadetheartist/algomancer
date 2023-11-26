use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase, CardPrototypeId};
use crate::game::state::card::Card;
use crate::game::state::card_collection::{CardCollectionId, FindCardCollectionMutResult};
use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutator;
use crate::game::state::player::PlayerId;
use crate::game::state::State;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdatePlayerHealthMutation {
    pub player_id: PlayerId,
    pub new_value: i32,
}

impl StateMutator for UpdatePlayerHealthMutation {
    fn mutate_state(&self, mut state: State, db: &CardPrototypeDatabase) -> Result<State, StateError> {
        state.find_player_mut(self.player_id)?.health = self.new_value;
        Ok(state)
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdatePlayerAliveMutation {
    pub player_id: PlayerId,
    pub new_value: bool,
}

impl StateMutator for UpdatePlayerAliveMutation {
    fn mutate_state(&self, mut state: State, db: &CardPrototypeDatabase) -> Result<State, StateError> {
        state.find_player_mut(self.player_id)?.is_alive = self.new_value;
        Ok(state)
    }
}
