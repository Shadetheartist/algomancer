use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase};
use crate::game::state::card::CardId;
use crate::game::state::card_collection::CardCollectionId;
use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutator;
use crate::game::state::State;



#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum DeckPlacement {
    OnTop,
    OnBottom,
    ToIndex(usize),
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct MoveCardMutation {
    pub from_cc_id: CardCollectionId,
    pub to_cc_id: CardCollectionId,
    pub card_id: CardId,
    pub placement: Option<DeckPlacement>,
}

impl StateMutator for MoveCardMutation {
    fn mutate_state(&self, mut state: State, _db: &CardPrototypeDatabase) -> Result<State, StateError> {
        let card = {
            let from_cc = state.find_card_collection_mut(self.from_cc_id)?;
            from_cc.remove(self.card_id)?
        };

        let to_cc = state.find_card_collection_mut(self.to_cc_id)?;
        if let Some(placement) = &self.placement {
            match placement {
                DeckPlacement::OnBottom => { to_cc.add_to_bottom(card)?; }
                DeckPlacement::OnTop => { todo!() }
                DeckPlacement::ToIndex(_) => { todo!() }
            }
        } else {
            to_cc.add(card);
        }

        Ok(state)
    }
}
