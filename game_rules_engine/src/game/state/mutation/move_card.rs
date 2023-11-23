use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase};
use crate::game::state::card::CardId;
use crate::game::state::card_collection::{CardCollectionId, FindCardCollectionMutResult};
use crate::game::state::error::{CardCollectionError, StateError};
use crate::game::state::mutation::StateMutator;
use crate::game::state::State;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Placement {
    OnTop,
    OnBottom,
    ToIndex(usize),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum To {
    Ordered(CardCollectionId, Placement),
    Unordered(CardCollectionId)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MoveCardMutation {
    pub from: CardCollectionId,
    pub to: To,
    pub card_id: CardId,
}

impl StateMutator for MoveCardMutation {
    fn mutate_state(&self, mut state: State, _db: &CardPrototypeDatabase) -> Result<State, StateError> {

        let card = {
            let from_cc = state.find_card_collection_mut(self.from)?;
            match from_cc {
                FindCardCollectionMutResult::CommonDeck(cc) => { cc.remove(self.card_id)? }
                FindCardCollectionMutResult::PlayerHand(_, cc) => { cc.remove(self.card_id)? }
                FindCardCollectionMutResult::PlayerDiscard(_, cc) => { cc.remove(self.card_id)? }
                FindCardCollectionMutResult::PlayerDeck(_, cc) => { cc.remove(self.card_id)? }
                FindCardCollectionMutResult::PlayerPack(_, cc) => { cc.remove(self.card_id)? }
            }
        };

        match &self.to {
            To::Ordered(id, placement) => {
                let to_cc = state.find_card_collection_mut(*id)?;

                match to_cc {
                    FindCardCollectionMutResult::PlayerDeck(_, cc) |
                    FindCardCollectionMutResult::CommonDeck(cc) => {
                        match placement {
                            Placement::OnTop => {
                                cc.add_to_top(card)
                            }
                            Placement::OnBottom => {
                                cc.add_to_bottom(card)
                            }
                            Placement::ToIndex(idx) => {
                               cc.insert(*idx, card)
                            }
                        }
                    }
                    FindCardCollectionMutResult::PlayerHand(_, _) |
                    FindCardCollectionMutResult::PlayerDiscard(_, _) |
                    FindCardCollectionMutResult::PlayerPack(_, _) => {
                        return Err(CardCollectionError::UnexpectedOrdering(*id).into())
                    }
                }
            }
            To::Unordered(id) => {
                let to_cc = state.find_card_collection_mut(*id)?;
                match to_cc {

                    FindCardCollectionMutResult::PlayerHand(_, cc) |
                    FindCardCollectionMutResult::PlayerDiscard(_, cc) |
                    FindCardCollectionMutResult::PlayerPack(_, cc) => {
                        cc.add(card)
                    }

                    FindCardCollectionMutResult::CommonDeck(_) |
                    FindCardCollectionMutResult::PlayerDeck(_, _) => {
                        return Err(CardCollectionError::UnexpectedOrdering(*id).into())
                    }
                }
            }
        }

        Ok(state)
    }
}
