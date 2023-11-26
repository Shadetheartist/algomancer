use serde::{Deserialize, Serialize};
use crate::game::action::{Action, ActionTrait, ActionType};
use crate::game::db::CardPrototypeDatabase;

use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutation};
use crate::game::state::player::{Player};
use crate::game::state::stack::Next;
use crate::game::state::State;
use crate::{sm_eval, sm_static};
use crate::game::state::mutation::stack_pass_priority::StackPassPriorityMutation;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct PassPriorityAction {}

impl ActionTrait for PassPriorityAction {
    fn generate_mutations(&self, state: &State, _db: &CardPrototypeDatabase, issuer: &Player) -> Result<Vec<StateMutation>, StateError> {
        let mut mutations = Vec::new();

        let player = issuer;
        let region = state.find_region_containing_player(player.id)?;

        // consider what the next stack item is
        match region.stack.next() {
            // if there is nothing left on the stack, we transition to the next step
            Next::TransitionStep => {
                // need to check if the current step is a globally synchronized step.
                // if so, we dont execute this mutation until all regions reach the same step.
                let all_regions_ready_to_transition = state.regions.iter().all(|r| r.step.is_global_sync_step());
                if region.step.is_global_sync_step() && all_regions_ready_to_transition {
                    // when the last region to reach the step arrives, we move all regions in the game to the next step.
                    for r in &state.regions {
                        mutations.extend(state.generate_mutations_for_phase_transition(r.id));
                    }
                } else {
                    mutations.extend(state.generate_mutations_for_phase_transition(region.id));
                }
            }
            Next::PassPriority(_) => {
                mutations.push(sm_static!(StackPassPriority, StackPassPriorityMutation{
                    region_id: region.id
                }));

                let region_id = region.id;
                let player_id = player.id;
                let eval = sm_eval!(move |future_state| {
                    let region = future_state.find_region(region_id)?;
                        let player = future_state.find_player(player_id)?;
                        if region.step.is_team_sync_step() {
                            let mut sub_mutations: Vec<StateMutation> = Vec::new();

                            // for team sync steps, we move all regions together to the next step
                            // once all players on a team have passed priority

                            let all_players_passed_priority = future_state.players_on_team(player.team_id)?.into_iter().all(|p| {
                                let p_region = future_state.find_region_containing_player(p.id).expect("a region");
                                if let Next::TransitionStep = p_region.stack.next() {
                                    true
                                } else {
                                    false
                                }
                            });

                            if all_players_passed_priority {
                                for r in &future_state.regions {
                                    sub_mutations.extend(future_state.generate_mutations_for_phase_transition(r.id));
                                }
                            }

                            Ok(Some(StateMutation::Vec(sub_mutations)))
                        } else {
                            Ok(None)
                        }
                });

                mutations.push(eval);
            }
            Next::ResolveEffect(_) => {
                panic!("idk what to do here");
            }
        }

        Ok(mutations)
    }

    fn get_valid(state: &State, _db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions = Vec::new();

        for region in &state.regions {
            for player in &region.players {
                match region.step {
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


