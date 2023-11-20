use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase, CardPrototypeId};
use crate::game::state::card::Card;
use crate::game::state::card_collection::{CardCollectionId, FindCardCollectionMutResult};
use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutator;
use crate::game::state::State;

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct CreateCardMutation {
    pub card_collection_id: CardCollectionId,
    pub card_prototype_id: CardPrototypeId
}

impl StateMutator for CreateCardMutation {
    fn mutate_state(&self, mut state: State, db: &CardPrototypeDatabase) -> Result<State, StateError> {
        let card = Card::from_prototype_id(db, &mut state, self.card_prototype_id);
        let cc = state.find_card_collection_mut(self.card_collection_id)?;

        match cc {
            FindCardCollectionMutResult::CommonDeck(cc) |
            FindCardCollectionMutResult::PlayerDeck(_, cc) => {
                cc.add_to_top(card)
            }
            FindCardCollectionMutResult::PlayerHand(_, cc) |
            FindCardCollectionMutResult::PlayerDiscard(_, cc) |
            FindCardCollectionMutResult::PlayerPack(_, cc) => {
                cc.add(card)
            }
        }

        Ok(state)
    }
}
