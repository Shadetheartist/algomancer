use serde::{Deserialize, Serialize};
use database::{CardPrototypeDatabase};

use crate::game::state::card_collection::{CardCollectionId};
use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutator;
use crate::game::state::player::PlayerId;
use crate::game::state::State;
use crate::game::state::unordered_cards::UnorderedCards;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreatePackMutation {
    pub player_id: PlayerId,
}

impl StateMutator for CreatePackMutation {
    fn mutate_state(&self, mut state: State, _: &CardPrototypeDatabase) -> Result<State, StateError> {
        let player = state.find_player_mut(self.player_id)?;
        player.pack = Some(UnorderedCards::new(CardCollectionId::new_pack(self.player_id)));

        Ok(state)
    }
}
