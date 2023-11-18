use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase};

use crate::game::state::card_collection::{CardCollection};
use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutator;
use crate::game::state::player::PlayerId;
use crate::game::state::State;

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct CreatePackMutation {
    pub player_id: PlayerId,
}

impl StateMutator for CreatePackMutation {
    fn mutate_state(&self, mut state: State, _: &CardPrototypeDatabase) -> Result<State, StateError> {
        let player = state.find_player_mut(self.player_id)?;
        player.pack = Some(CardCollection::new_pack(self.player_id));

        Ok(state)
    }
}
