

use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase, CardPrototypeId};


use crate::game::state::card::{Card, CardId, };
use crate::game::state::card_collection::{CardCollection, CardCollectionId};
use crate::game::state::error::StateError;
use crate::game::state::mutation::StaticStateMutation::{PhaseTransition, SetPlayerPassedPriority};
use crate::game::state::player::PlayerId;
use crate::game::state::region::RegionId;
use crate::game::state::State;

pub type StateMutationEvaluator = dyn Fn(&State) -> Result<StateMutation, StateError>;

pub enum StateMutation {
    Static(StaticStateMutation),
    Eval(Box<StateMutationEvaluator>),
}

impl StateMutation {
    pub fn to_static(self, state: &State) -> Result<StaticStateMutation, StateError> {
        match self {
            StateMutation::Static(static_mutation) => {
                Ok(static_mutation)
            }
            StateMutation::Eval(eval_fn) => {
                let evaluated = (eval_fn)(state)?;
                evaluated.to_static(state)
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
        placement: Option<DeckPlacement>
    },
    CreatePackForPlayer { player_id: PlayerId },
    CreateCard {
        cc_id: CardCollectionId,
        card_prototype_id: CardPrototypeId,
    }
}



impl State {
    pub fn mutate(self, db: &CardPrototypeDatabase, state_mutation: &StaticStateMutation) -> Result<State, StateError> {
        match state_mutation {
            mutation @ StaticStateMutation::SetPlayerPassedPriority { .. } => self.handle_set_player_passed_priority(mutation),
            mutation @ StaticStateMutation::PhaseTransition { .. } => self.handle_phase_transition(mutation),
            mutation @ StaticStateMutation::MoveCard { .. } => self.handle_move_card(mutation),
            mutation @ StaticStateMutation::CreatePackForPlayer { .. } => self.handle_create_pack(mutation),
            mutation @ StaticStateMutation::CreateCard { .. } => self.handle_create_card(db, mutation),
        }
    }

    fn handle_create_card(mut self, db: &CardPrototypeDatabase, state_mutation: &StaticStateMutation) -> Result<State, StateError> {
        if let StaticStateMutation::CreateCard { cc_id, card_prototype_id } = state_mutation {
            let card = Card::from_prototype_id(db, &mut self, *card_prototype_id);

            let cc = self.find_card_collection_mut(*cc_id)?;
            cc.add(card);

            Ok(self)
        } else {
            panic!("only call this for StateMutation::MoveCard")
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
        if let StaticStateMutation::MoveCard { from_cc_id, to_cc_id, card_id, placement } = state_mutation {
            let card = {
                let from_cc = self.find_card_collection_mut(*from_cc_id)?;
                from_cc.remove(*card_id)?
            };

            let to_cc = self.find_card_collection_mut(*to_cc_id)?;
            if let Some(placement) = placement {
                match placement {
                    DeckPlacement::OnBottom => { to_cc.add_to_bottom(card)?; }
                    DeckPlacement::OnTop => { todo!() }
                    DeckPlacement::ToIndex(_) => { todo!() }
                }
            } else {
                to_cc.add(card);
            }

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

    pub fn generate_mutations_for_phase_transition(&self, region_id: RegionId) -> Vec<StateMutation> {
        let mut mutations = Vec::new();

        mutations.push(StateMutation::Static(PhaseTransition { region_id }));

        for p in self.players_in_region(region_id).expect("players") {
            mutations.push(StateMutation::Static(SetPlayerPassedPriority { player_id: p.id, value: false }));
        }

        mutations
    }
}


#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum DeckPlacement {
    OnTop,
    OnBottom,
    ToIndex(usize),
}
