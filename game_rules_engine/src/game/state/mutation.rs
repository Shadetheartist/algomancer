pub mod create_card;
pub mod create_pack;
pub mod move_card;
pub mod phase_transition;
pub mod pass_priority;
pub mod stack_pass_priority;
pub mod stack_add_priority;
pub mod stack_clear_priority;
pub mod player_mutations;
pub mod remove_card;
pub mod create_permanent;
pub mod set_resource_tapped;

use std::fmt::{Debug};
use serde::{Deserialize, Serialize};
use crate::{phase_transition, stack_add_priority, stack_clear_priority};
use database::{CardPrototypeDatabase};
use crate::game::state::error::StateError;
use crate::game::state::mutation::create_card::CreateCardMutation;
use crate::game::state::mutation::create_pack::CreatePackMutation;
use crate::game::state::mutation::create_permanent::CreatePermanentMutation;
use crate::game::state::mutation::move_card::MoveCardMutation;
use crate::game::state::mutation::phase_transition::PhaseTransitionMutation;
use crate::game::state::mutation::player_mutations::{UpdatePlayerAliveMutation, UpdatePlayerHealthMutation, UpdatePlayerResourcesPlayedMutation};
use crate::game::state::mutation::remove_card::RemoveCardMutation;
use crate::game::state::mutation::stack_add_priority::StackAddPriorityMutation;
use crate::game::state::mutation::stack_clear_priority::StackClearPriorityMutation;
use crate::game::state::mutation::stack_pass_priority::StackPassPriorityMutation;
use crate::game::state::mutation::set_resource_tapped::SetResourceTappedMutation;
use crate::game::state::region::RegionId;
use crate::game::state::State;


pub trait StateMutator {
    fn mutate_state(&self, state: State, db: &CardPrototypeDatabase) -> Result<State, StateError>;
}

pub type StateMutationEvaluator = dyn Fn(&State) -> Result<Option<StateMutation>, StateError>;
pub type StateMutationEvaluatorVec = dyn Fn(&State) -> Result<Vec<StateMutation>, StateError>;

pub enum StateMutation {
    Static(StaticStateMutation),
    Vec(Vec<StateMutation>),
    Eval(Box<StateMutationEvaluator>),
    EvalVec(Box<StateMutationEvaluatorVec>),
}

impl StateMutation {
    pub fn to_static(self, state: &State) -> Result<Vec<StaticStateMutation>, StateError> {
        match self {
            StateMutation::Static(static_mutation) => {
                Ok(vec![static_mutation])
            }
            StateMutation::Vec(mutations) => {
                let mut statics = Vec::new();
                for m in mutations {
                    statics.extend(m.to_static(state)?)
                }
                Ok(statics)
            }
            StateMutation::Eval(eval_fn) => {
                let state_mutation = (eval_fn)(state)?;
                match state_mutation {
                    None => {
                        Ok(vec![])
                    }
                    Some(evaluated) => {
                        evaluated.to_static(state)
                    }
                }
            }
            StateMutation::EvalVec(eval_fn) => {
                let state_mutation = (eval_fn)(state)?;
                let mut statics = Vec::new();
                for m in state_mutation {
                    statics.extend(m.to_static(state)?)
                }
                Ok(statics)
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
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum StaticStateMutation {
    StackClearPriority(StackClearPriorityMutation),
    StackAddPriority(StackAddPriorityMutation),
    StackPassPriority(StackPassPriorityMutation),
    PhaseTransition(PhaseTransitionMutation),
    MoveCard(MoveCardMutation),
    CreatePackForPlayer(CreatePackMutation),
    CreateCard(CreateCardMutation),
    RemoveCard(RemoveCardMutation),
    UpdatePlayerHealth(UpdatePlayerHealthMutation),
    UpdatePlayerAlive(UpdatePlayerAliveMutation),
    UpdatePlayerResourcesPlayed(UpdatePlayerResourcesPlayedMutation),
    CreatePermanent(CreatePermanentMutation),
    SetResourceTapped(SetResourceTappedMutation),
}



impl State {
    pub fn mutate(self, db: &CardPrototypeDatabase, state_mutation: &StaticStateMutation) -> Result<State, StateError> {
        match state_mutation {
            StaticStateMutation::StackClearPriority(m) => m.mutate_state(self, db),
            StaticStateMutation::StackAddPriority(m) => m.mutate_state(self, db),
            StaticStateMutation::StackPassPriority(m) => m.mutate_state(self, db),
            StaticStateMutation::PhaseTransition(m) => m.mutate_state(self, db),
            StaticStateMutation::MoveCard(m) => m.mutate_state(self, db),
            StaticStateMutation::CreatePackForPlayer(m) => m.mutate_state(self, db),
            StaticStateMutation::CreateCard(m) => m.mutate_state(self, db),
            StaticStateMutation::RemoveCard(m) => m.mutate_state(self, db),
            StaticStateMutation::UpdatePlayerHealth(m) => m.mutate_state(self, db),
            StaticStateMutation::UpdatePlayerAlive(m) => m.mutate_state(self, db),
            StaticStateMutation::UpdatePlayerResourcesPlayed(m) => m.mutate_state(self, db),
            StaticStateMutation::CreatePermanent(m) => m.mutate_state(self, db),
            StaticStateMutation::SetResourceTapped(m) => m.mutate_state(self, db),
        }
    }
}



impl State {
    pub fn generate_mutation_for_phase_transition(&self, region_id: RegionId) -> StateMutation {
        let mut mutations = Vec::new();

        let next_phase = self.find_region(region_id).expect("a region").step.get_next_phase(&self.game_mode);

        phase_transition!(mutations, region_id, next_phase);

        stack_clear_priority!(mutations, region_id);

        for p in self.players_in_region(region_id).expect("players") {
            // these should be pushed in IT order
            stack_add_priority!(mutations, region_id, p.id);
        }

        StateMutation::Vec(mutations)
    }
}


#[macro_export]
macro_rules! sm_static {
    ($sm_enum:ident, $arg:expr) => {
        $crate::game::state::mutation::StateMutation::Static(
            $crate::game::state::mutation::StaticStateMutation::$sm_enum($arg)
        )
    };
}

#[macro_export]
macro_rules! sm_vec {
    ($items:expr) => {
        $crate::game::state::mutation::StateMutation::Vec(vec![$items])
    };
}


#[macro_export]
macro_rules! sm_eval {
    ($func:expr) => {
        $crate::game::state::mutation::StateMutation::Eval(Box::new($func))
    };
}

#[macro_export]
macro_rules! sm_eval_vec {
    ($func:expr) => {
        $crate::game::state::mutation::StateMutation::EvalVec(Box::new($func))
    };
}