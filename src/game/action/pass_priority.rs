use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::State;

impl Game {
    pub fn apply_pass_priority_action(&self, state: &mut State, action: &Action) {
        if let Action::PassPriority(player_id) = action {
            let region_id = state.region_id_containing_player(*player_id);
            let player = state.player_mut(*player_id).expect("the player that passed priority");
            player.passed_priority = true;

            println!("Player [{:?}] passed priority in region [{:?}]", &action, region_id);

            if state.all_players_in_region_passed_priority(region_id) {
                state.transition_to_next_step(region_id);
            }
        } else {
            panic!("action should have been pass priority")
        }
    }
}
