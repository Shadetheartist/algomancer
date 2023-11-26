use crate::game::state::mutation::player_mutations::{UpdatePlayerAliveMutation};
use crate::game::state::mutation::StateMutation;
use crate::game::state::stack::Next;
use crate::game::state::State;
use crate::{sm_static};
use crate::game::state::progression::{CombatPhaseStep, Phase};

impl State {
    pub fn generate_state_based_mutations(&self) -> Vec<StateMutation> {
        let mut mutations = Vec::new();

        mutations = add_sba_transition(self, mutations);
        mutations = add_sba_player(self, mutations);
        mutations = add_sba_damage(self, mutations);

        mutations
    }
}

fn add_sba_transition(state: &State, mut mutations: Vec<StateMutation>) -> Vec<StateMutation> {

    // for team sync steps, we move all regions together to the next step
    // once all players on a team have passed priority

    for r in &state.regions {
        let next  = r.stack.next();
        match next {
            Next::TransitionStep => {
                if r.step.is_team_sync_step() {

                    let active_team_id = r.active_team_id(state).expect("an active team in the region");

                    let all_players_on_team_passed_priority = state.players_on_team(active_team_id).expect("players on the team").into_iter().all(|p| {
                        let p_region = state.find_region_containing_player(p.id).expect("a region");
                        if let Next::TransitionStep = p_region.stack.next() {
                            true
                        } else {
                            false
                        }
                    });

                    if all_players_on_team_passed_priority {
                        for r in &state.regions {
                            mutations.push(state.generate_mutation_for_phase_transition(r.id));
                        }
                    }
                } else {
                    mutations.push(state.generate_mutation_for_phase_transition(r.id));
                }
            }
            Next::PassPriority(_) => {
            }
            Next::ResolveEffect(_) => {
            }
        }
    }

    mutations
}

fn add_sba_player(state: &State, mutations: Vec<StateMutation>) -> Vec<StateMutation> {
    for r in &state.regions {
        for p in &r.players {
            if !p.is_alive {
                // if the player is already dead they can't do anything
                continue;
            }

            if p.health == 0 {
                sm_static!(UpdatePlayerAlive, UpdatePlayerAliveMutation {
                    player_id: p.id,
                    new_value: false,
                });

                //todo: remove the players possessions from the game?

                // if the player died we don't really care what else happens to them
                continue;
            }
        }
    }

    mutations
}

fn add_sba_damage(state: &State, mut mutations: Vec<StateMutation>) -> Vec<StateMutation> {
    for r in &state.regions {
        match r.step {
            Phase::CombatPhaseA(CombatPhaseStep::Damage) | Phase::CombatPhaseB(CombatPhaseStep::Damage) => {
                mutations.push(state.generate_mutation_for_phase_transition(r.id));
            }
            _ => {}
        }
    }

    mutations
}