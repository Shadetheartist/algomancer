pub mod create_card;
pub mod create_pack;
pub mod move_card;
pub mod phase_transition;
pub mod set_player_passed_priority;

use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase};
use crate::game::state::error::StateError;
use crate::game::state::mutation::create_card::CreateCardMutation;
use crate::game::state::mutation::create_pack::CreatePackMutation;
use crate::game::state::mutation::move_card::MoveCardMutation;
use crate::game::state::mutation::phase_transition::PhaseTransitionMutation;
use crate::game::state::mutation::set_player_passed_priority::SetPlayerPassedPriorityMutation;
use crate::game::state::region::RegionId;
use crate::game::state::State;


pub trait StateMutator {
    fn mutate_state(&self, state: State, db: &CardPrototypeDatabase) -> Result<State, StateError>;
}

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
    SetPlayerPassedPriority(SetPlayerPassedPriorityMutation),
    PhaseTransition(PhaseTransitionMutation),
    MoveCard(MoveCardMutation),
    CreatePackForPlayer(CreatePackMutation),
    CreateCard(CreateCardMutation)
}



impl State {
    pub fn mutate(self, db: &CardPrototypeDatabase, state_mutation: &StaticStateMutation) -> Result<State, StateError> {
        match state_mutation {
            StaticStateMutation::SetPlayerPassedPriority(m) => m.mutate_state(self, db),
            StaticStateMutation::PhaseTransition(m) => m.mutate_state(self, db),
            StaticStateMutation::MoveCard(m) => m.mutate_state(self, db),
            StaticStateMutation::CreatePackForPlayer(m) => m.mutate_state(self, db),
            StaticStateMutation::CreateCard(m) => m.mutate_state(self, db),
        }
    }
}



impl State {
    pub fn generate_mutations_for_phase_transition(&self, region_id: RegionId) -> Vec<StateMutation> {
        let mut mutations = Vec::new();

        mutations.push(StateMutation::Static(StaticStateMutation::PhaseTransition(PhaseTransitionMutation{
            region_id
        })));

        for p in self.players_in_region(region_id).expect("players") {
            mutations.push(StateMutation::Static(StaticStateMutation::SetPlayerPassedPriority(SetPlayerPassedPriorityMutation{
                player_id: p.id,
                value: false,
            })));
        }

        mutations
    }
}