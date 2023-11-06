use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::player::StateError;
use crate::game::state::progression::Phase::PrecombatPhase;
use crate::game::state::progression::PrecombatPhaseStep;
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

            if let PrecombatPhase(PrecombatPhaseStep::PassPack) = current_step {
                let region_ids : Vec<RegionId> = state.regions.iter().map(|r| r.region_id).collect();
                for region_id in region_ids {
                    state = state.region_transition_to_next_step(region_id);
                }

            } else {
                if state.all_players_in_region_passed_priority(region_id)? {
                    state = state.region_transition_to_next_step(region_id);
                }
            }

            Ok(state)
        } else {
            panic!("action should have been pass priority")
        }
    }
}
