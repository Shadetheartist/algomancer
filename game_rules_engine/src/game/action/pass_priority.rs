use serde::{Deserialize, Serialize};
use crate::game::action::{Action, ActionTrait, ActionType};
use crate::game::db::CardPrototypeDatabase;

use crate::game::state::error::StateError;
use crate::game::state::mutation::{StateMutation, StaticStateMutation};
use crate::game::state::mutation::set_player_passed_priority::SetPlayerPassedPriorityMutation;
use crate::game::state::player::{Player, TeamId};
use crate::game::state::progression::{CombatPhaseAStep, Phase, PrecombatPhaseStep};
use crate::game::state::progression::Phase::PrecombatPhase;
use crate::game::state::region::{Region};
use crate::game::state::State;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct PassPriorityAction {}

impl ActionTrait for PassPriorityAction {
    fn generate_mutations(&self, state: &State, _db: &CardPrototypeDatabase, issuer: &Player) -> Result<Vec<StateMutation>, StateError> {
        let mut mutations = Vec::new();

        let player = issuer;
        let region: &Region = state.find_region_containing_player(player.id)?;


        mutations.push(StateMutation::Static(StaticStateMutation::SetPlayerPassedPriority(SetPlayerPassedPriorityMutation{
            player_id: player.id,
            value: true
        })));

        // transition only the region that the player occupies when all players in the region have passed
        let region_pass = |mutations: &mut Vec<StateMutation>| -> Result<(), StateError> {
            if state.all_players_in_region_except_passed_priority(region.id, player.id)? {
                mutations.append(&mut state.generate_mutations_for_phase_transition(region.id))
            }
            Ok(())
        };

        // transition all regions after all players on a team have passed
        let team_pass = |mutations: &mut Vec<StateMutation>, team_id: TeamId| -> Result<(), StateError> {
            // have to exclude the current player since the state hasn't changed yet (could also solve with an eval)
            if state.all_players_on_team_passed_priority_except(team_id, player.id)? {
                for r in &state.regions {
                    mutations.append(&mut state.generate_mutations_for_phase_transition(r.id))
                }
            }
            Ok(())
        };

        let initiative_team_id = state.initiative_team;
        let non_initiative_team_id = state.non_initiative_team();

        match region.step {
            PrecombatPhase(step) => match step {
                PrecombatPhaseStep::Untap |
                PrecombatPhaseStep::Draw |
                PrecombatPhaseStep::Draft => {
                    region_pass(&mut mutations)?
                },
                PrecombatPhaseStep::PassPack => {
                    // this is a fake sync step, when the last player finishes their draft,
                    // all regions automatically transition to ITMana
                }
                PrecombatPhaseStep::ITMana => {
                    team_pass(&mut mutations, initiative_team_id)?;
                }
                PrecombatPhaseStep::NITMana => {
                    team_pass(&mut mutations, non_initiative_team_id)?;
                }
            }
            Phase::CombatPhaseA(step) => {
                match step {
                    CombatPhaseAStep::ITAttack => {
                        team_pass(&mut mutations, initiative_team_id)?;
                    }
                    CombatPhaseAStep::AfterITAttackPriorityWindow => {
                        region_pass(&mut mutations)?
                    }
                    CombatPhaseAStep::NITBlock => {
                        region_pass(&mut mutations)?
                    }
                    CombatPhaseAStep::AfterNITBlockPriorityWindow => {
                        region_pass(&mut mutations)?
                    }
                    CombatPhaseAStep::Damage => {
                        // not an interactive step,
                        // state modifications for this step will happen
                        // automatically after the block window is over,
                        // then it will move the after combat step
                    }
                    CombatPhaseAStep::AfterCombatPriorityWindow => {
                        region_pass(&mut mutations)?
                    }
                }
            }
            Phase::CombatPhaseB(_) => {}
            Phase::MainPhase(_) => {}
        }


        Ok(mutations)
    }

    fn get_valid(state: &State, _db: &CardPrototypeDatabase) -> Vec<Action> {
        let mut actions = Vec::new();

        for player in state.players() {
            if state.player_can_act(player.id) {
                actions.push(Action {
                    issuer_player_id: player.id,
                    action: ActionType::PassPriority(PassPriorityAction{})
                })
            }
        }

        actions
    }
}


