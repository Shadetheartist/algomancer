use serde::{Deserialize, Serialize};
use crate::game::state::card::{CardId};
use crate::game::state::card_collection::{CardCollectionId};
use crate::game::state::player::StateError;
use crate::game::state::progression::{Phase};
use crate::game::state::region::RegionId;
use crate::game::state::State;

/// State mutations are an instruction to make the smallest meaningful change in state.
/// Actions, the next level up, generate a list state mutations, which are then applied to the
/// state in order.
///
/// This list of individual small changes in state can be serialized and sent to clients so
/// that they can coherently display what happened between the application of the last action
/// and the next state.
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum StateMutation {
    PhaseTransition { region_id: RegionId, phase: Phase },
    MoveCard {
        from_cc_id: CardCollectionId,
        to_cc_id: CardCollectionId,
        card_id: CardId,
    },
}


impl State {
    pub fn mutate(mut self, state_mutation: &StateMutation) -> Result<State, StateError> {
        match *state_mutation {
            StateMutation::PhaseTransition { region_id, phase } => self.handle_phase_transition(region_id, phase),
            StateMutation::MoveCard {
                from_cc_id,
                to_cc_id,
                card_id
            } => {
                self.handle_move_card(
                    from_cc_id,
                    to_cc_id,
                    card_id,
                )
            }
        }
    }

    fn handle_move_card(mut self, from_cc_id: CardCollectionId, to_cc_id: CardCollectionId, card_id: CardId) -> Result<State, StateError> {

        let card = {
            let from_cc = self.find_card_collection_mut(from_cc_id)?;
            from_cc.remove(card_id)?
        };

        let to_cc = self.find_card_collection_mut(to_cc_id)?;
        to_cc.add(card);

        Ok(self)
    }

    fn handle_phase_transition(mut self, region_id: RegionId, phase: Phase) -> Result<State, StateError> {

        todo!();

        Ok(self)
    }
}