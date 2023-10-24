use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::State;

impl Game {
    pub fn apply_pass_priority_action(&self, state: &mut State, action: &Action) {
        if let Action::PassPriority(player_id) = action {
            let player = player_id.get_player(state).expect("the player that passed priority");
            player.passed_priority = true;
            if state.all_players_passed_priority() {
                state.transition_to_next_step();
            }
        } else {
            panic!("action should have been pass priority")
        }
    }
}
