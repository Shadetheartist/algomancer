use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase, CardPrototypeId};
use crate::game::state::card::Card;
use crate::game::state::card_collection::{CardCollectionId, FindCardCollectionMutResult};
use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutator;
use crate::game::state::State;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateCardMutation {
    pub card_collection_id: CardCollectionId,
    pub card: Card
}

impl StateMutator for CreateCardMutation {
    fn mutate_state(&self, mut state: State, db: &CardPrototypeDatabase) -> Result<State, StateError> {

        state.next_card_id += 1;

        let cc = state.find_card_collection_mut(self.card_collection_id)?;

        match cc {
            FindCardCollectionMutResult::CommonDeck(cc) |
            FindCardCollectionMutResult::PlayerDeck(_, cc) => {
                cc.add_to_top(self.card.clone())
            }
            FindCardCollectionMutResult::PlayerHand(_, cc) |
            FindCardCollectionMutResult::PlayerDiscard(_, cc) |
            FindCardCollectionMutResult::PlayerPack(_, cc) => {
                cc.add(self.card.clone())
            }
        }

        Ok(state)
    }
}
