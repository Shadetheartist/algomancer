use crate::game::state::mutation::player_mutations::{UpdatePlayerAliveMutation, UpdatePlayerHealthMutation};
use crate::game::state::mutation::StateMutation;
use crate::game::state::stack::Next;
use crate::game::state::State;
use crate::{sm_eval, sm_eval_vec, sm_static, sm_vec};
use crate::game::state::progression::{CombatPhaseAStep, CombatPhaseBStep, Phase};

impl State {
    pub fn generate_state_based_mutations(&self) -> Vec<StateMutation> {
        let mut mutations = Vec::new();

        mutations = add_sba_player(self, mutations);
        mutations = add_sba_damage(self, mutations);

        mutations.push(
            sm_eval_vec!(move |state| {
                let mut mutations = Vec::new();
                for r in &state.regions {
                    match r.stack.next() {
                        Next::TransitionStep => {
                            mutations.push(state.generate_mutation_for_phase_transition(r.id));
                        }
                        Next::PassPriority(_) => {
                        }
                        Next::ResolveEffect(_) => {
                        }
                    }
                }

                Ok(mutations)
            })
        );


        mutations
    }
}

fn add_sba_player(state: &State, mut mutations: Vec<StateMutation>) -> Vec<StateMutation> {
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
            Phase::CombatPhaseA(CombatPhaseAStep::Damage) |  Phase::CombatPhaseB(CombatPhaseBStep::Damage) => {
                eprintln!("Doing damage...");
            }
            _ => {}
        }
    }

    mutations
}