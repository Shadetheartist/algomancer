use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::error::StateError;
use crate::game::state::mutation::StateMutation;
use crate::game::state::mutation::StaticStateMutation::{PhaseTransition, SetPlayerPassedPriority};
use crate::game::state::player::TeamId;
use crate::game::state::progression::{CombatPhaseAStep, Phase, PrecombatPhaseStep};
use crate::game::state::progression::Phase::PrecombatPhase;
use crate::game::state::region::Region;

impl Game {
    pub fn generate_pass_priority_state_mutations(&self, action: &Action) -> Result<Vec<StateMutation>, StateError> {
        if let Action::PassPriority(player_id) = action {
            let mut mutations = Vec::new();

            let state = &self.state;
            let player = state.find_player(*player_id)?;
            let region: &Region = state.find_region_containing_player(player.player_id)?;

            mutations.push(StateMutation::Static(SetPlayerPassedPriority { player_id: player.player_id, value: true }));

            /// transition only the region that the player occupies when all players in the region have passed
            let region_pass = |mutations: &mut Vec<StateMutation>| -> Result<(), StateError> {
                if state.all_players_in_region_except_passed_priority(region.region_id, player.player_id)? {
                    mutations.push(StateMutation::Static(PhaseTransition { region_id: region.region_id }))
                }
                Ok(())
            };

            /// transition all regions after all players on a team have passed
            let team_pass = |mutations: &mut Vec<StateMutation>, team_id: TeamId| -> Result<(), StateError> {
                if state.all_players_on_team_passed_priority(team_id)? {
                    for r in &state.regions {
                        mutations.push(StateMutation::Static(PhaseTransition { region_id: r.region_id }))
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
        } else {
            panic!("only call this for pass priority actions")
        }
    }
}
