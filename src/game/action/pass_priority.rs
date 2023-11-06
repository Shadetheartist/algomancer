use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::player::{StateError, TeamId};
use crate::game::state::progression::Phase::PrecombatPhase;
use crate::game::state::progression::{Phase, PrecombatPhaseStep};
use crate::game::state::region::RegionId;
use crate::game::state::State;

impl Game {
    pub fn apply_pass_priority_action(&self, mut state: State, action: &Action) -> Result<State, StateError> {
        if let Action::PassPriority(player_id) = action {

            let (region_id, player_id) = state.find_region_containing_player(*player_id).expect("a region with player");
            {
                let player = state.find_player_mut(player_id).expect("a player");
                player.passed_priority = true;
            }

            let current_step = state.find_region(region_id).expect("a region").step.clone();

            // transition only the region that the player occupies when all players in the region have passed
            fn region_pass(mut state: State, region_id: RegionId) -> Result<State, StateError> {
                if state.all_players_in_region_passed_priority(region_id)? {
                    state = state.region_transition_to_next_step(region_id);
                }
                Ok(state)
            }

            // transition all regions after all players on a team have passed
            fn team_pass(mut state: State, team_id: TeamId) -> Result<State, StateError> {
                if state.all_players_on_team_passed_priority(team_id)? {
                    state = transition_all_regions(state)
                }
                Ok(state)
            }

            fn transition_all_regions(mut state: State) -> State {
                let region_ids : Vec<RegionId> = state.regions.iter().map(|r| r.region_id).collect();

                for region_id in region_ids {
                    state = state.region_transition_to_next_step(region_id);
                }

                state
            }

            match current_step {
                PrecombatPhase(step) => match step {
                    PrecombatPhaseStep::Untap |
                    PrecombatPhaseStep::Draw |
                    PrecombatPhaseStep::Draft => {
                        state = region_pass(state, region_id)?
                    },
                    PrecombatPhaseStep::PassPack => {
                        state = transition_all_regions(state)
                    }
                    PrecombatPhaseStep::ITMana => {
                        let team_id = state.initiative_team;
                        state = team_pass(state, team_id)?
                    }
                    PrecombatPhaseStep::NITMana => {
                        let team_id = state.non_initiative_team();
                        state = team_pass(state, team_id)?
                    }

                }
                Phase::CombatPhaseA(_) => {}
                Phase::CombatPhaseB(_) => {}
                Phase::MainPhase(_) => {}
            }

            Ok(state)
        } else {
            panic!("action should have been pass priority")
        }
    }
}
