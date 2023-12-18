use serde::{Deserialize, Serialize};
use database::{CardPrototypeDatabase};
use crate::game::state::card::{CardId, FindCardResult};

use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutator;
use crate::game::state::State;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RemoveCardMutation {
    pub card_id: CardId,
}

impl StateMutator for RemoveCardMutation {
    fn mutate_state(&self, mut state: State, _db: &CardPrototypeDatabase) -> Result<State, StateError> {
        let result = state.find_card(self.card_id)?;
        match result {
            FindCardResult::InPlayerHand(p, _, _) => {
                let p = state.find_player_mut(p.id)?;
                p.hand.remove(self.card_id)?;
            }
            FindCardResult::InPlayerDiscard(p, _, _) => {
                let p = state.find_player_mut(p.id)?;
                p.discard.remove(self.card_id)?;
            }
            _ => { panic!("not supported" )}
        }

        Ok(state)
    }
}
