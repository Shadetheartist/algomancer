use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::player::StateError;
use crate::game::state::State;

impl Game {
    pub fn apply_pass_priority_action(&self, mut state: State, action: &Action) -> Result<State, StateError> {
        if let Action::PassPriority(player_id) = action {
            let (region_id, player_id) = state.find_region_containing_player(*player_id).expect("a region with player");

            {
                let player = state.find_player_mut(player_id).expect("a player");
                player.passed_priority = true;
            }

            if state.all_players_in_region_passed_priority(region_id)? {
                state = state.region_transition_to_next_step(region_id);
            }

            Ok(state)
        } else {
            panic!("action should have been pass priority")
        }
    }
}
