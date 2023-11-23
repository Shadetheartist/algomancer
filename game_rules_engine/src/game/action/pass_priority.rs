use serde::{Deserialize, Serialize};
use crate::game::action::{Action, ActionTrait, ActionType};
use crate::game::db::CardPrototypeDatabase;

use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutation, StateMutationEvaluator, StaticStateMutation};
use crate::game::state::mutation::stack_pass_priority::StackPassPriorityMutation;
use crate::game::state::player::{Player, TeamId};
use crate::game::state::progression::{CombatPhaseAStep, Phase, PrecombatPhaseStep};
use crate::game::state::progression::Phase::PrecombatPhase;
use crate::game::state::stack::Next;
use crate::game::state::State;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct PassPriorityAction {}

impl ActionTrait for PassPriorityAction {
    fn generate_mutations(&self, state: &State, _db: &CardPrototypeDatabase, issuer: &Player) -> Result<Vec<StateMutation>, StateError> {
        let mut mutations = Vec::new();

        let player = issuer;
        let region = state.find_region_containing_player(player.id)?;

        match region.stack.next() {
            Next::TransitionStep => {
                mutations.extend(state.generate_mutations_for_phase_transition(region.id))
            }
            Next::PassPriority(passing_player) => {
                mutations.push(StateMutation::Static(StaticStateMutation::StackPassPriority(StackPassPriorityMutation { region_id: region.id })))
            }
            Next::ResolveEffect(_) => {
                panic!("idk what to do here")
            }
        }

        let player_id = player.id;
        StateMutation::Eval(Box::new(move |future_state| {
            let region = future_state.find_region_containing_player(player_id)?;
            Ok(None)
        }));

        Ok(mutations)
    }

    fn get_valid(state: &State, _db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions = Vec::new();

        for region in &state.regions {
            for player in &region.players {
                match region.step {
                    PrecombatPhase(PrecombatPhaseStep::Draft) => {}
                    _ => {
                        if state.player_can_act(player.id) {
                            actions.push(Action {
                                issuer_player_id: player.id,
                                action: ActionType::PassPriority(PassPriorityAction {}),
                            })
                        }
                    }
                }
            }
        }

        actions
    }
}


