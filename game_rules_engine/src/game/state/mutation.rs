

use serde::{Deserialize, Serialize};


use crate::game::state::card::CardId;
use crate::game::state::card_collection::{CardCollection, CardCollectionId};
use crate::game::state::error::StateError;
use crate::game::state::player::PlayerId;
use crate::game::state::region::RegionId;
use crate::game::state::State;

mod player;


pub enum StateMutation {
    Static(StaticStateMutation),
    Eval(Box<dyn Fn(&State) -> Result<StaticStateMutation, StateError>>),
}

impl StateMutation {
    pub fn to_static(self, state: &State) -> Result<StaticStateMutation, StateError> {
        match self {
            StateMutation::Static(static_mutation) => {
                Ok(static_mutation)
            }
            StateMutation::Eval(eval_fn) => {
                (eval_fn)(state)
            }
        }
    }
}


/// State mutations are an instruction to make the smallest meaningful change in state.
/// Actions, the next level up, generate a list state mutations, which are then applied to the
/// state in order.
///
/// This list of individual small changes in state can be serialized and sent to clients so
/// that they can coherently display what happened between the application of the last action
/// and the next state.
#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum StaticStateMutation {
    SetPlayerPassedPriority { player_id: PlayerId, value: bool },
    PhaseTransition { region_id: RegionId },
    MoveCard {
        from_cc_id: CardCollectionId,
        to_cc_id: CardCollectionId,
        card_id: CardId,
    },
    CreatePackForPlayer { player_id: PlayerId },
}



impl State {
    pub fn mutate(self, state_mutation: &StaticStateMutation) -> Result<State, StateError> {
        match state_mutation {
            mutation @ StaticStateMutation::SetPlayerPassedPriority { .. } => self.handle_set_player_passed_priority(mutation),
            mutation @ StaticStateMutation::PhaseTransition { .. } => self.handle_phase_transition(mutation),
            mutation @ StaticStateMutation::MoveCard { .. } => self.handle_move_card(mutation),
            mutation @ StaticStateMutation::CreatePackForPlayer { .. } => self.handle_create_pack(mutation),
        }
    }

    fn handle_set_player_passed_priority(mut self, state_mutation: &StaticStateMutation) -> Result<State, StateError> {
        if let StaticStateMutation::SetPlayerPassedPriority { player_id, value } = *state_mutation {
            let player = self.find_player_mut(player_id)?;
            player.passed_priority = value;
            Ok(self)
        } else {
            panic!("only call this for StateMutation::MoveCard")
        }
    }

    fn handle_move_card(mut self, state_mutation: &StaticStateMutation) -> Result<State, StateError> {
        if let StaticStateMutation::MoveCard { from_cc_id, to_cc_id, card_id } = *state_mutation {
            let card = {
                let from_cc = self.find_card_collection_mut(from_cc_id)?;
                from_cc.remove(card_id)?
            };

            let to_cc = self.find_card_collection_mut(to_cc_id)?;
            to_cc.add(card);

            Ok(self)
        } else {
            panic!("only call this for StateMutation::MoveCard")
        }
    }

    fn handle_phase_transition(self, state_mutation: &StaticStateMutation) -> Result<State, StateError> {
        if let StaticStateMutation::PhaseTransition { region_id } = *state_mutation {
            Ok(self.region_transition_to_next_step(region_id))
        } else {
            panic!("only call this for StateMutation::PhaseTransition")
        }
    }

    fn handle_create_pack(mut self, state_mutation: &StaticStateMutation) -> Result<State, StateError> {
        if let StaticStateMutation::CreatePackForPlayer { player_id } = *state_mutation {
            let player = self.find_player_mut(player_id)?;
            player.pack = Some(CardCollection::new_pack(player_id));
            Ok(self)
        } else {
            panic!("only call this for StateMutation::PhaseTransition")
        }
    }
}