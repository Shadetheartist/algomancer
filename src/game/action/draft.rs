use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::State;

impl Game {
    pub fn apply_draft_action(&self, state: &mut State, action: &Action) {
        if let Action::Draft { player_id, .. } = action {

            let player = player_id.get_player(state).unwrap();

            // todo: constructed vs draft (constructed draw and draft are combined)
            // todo: set hand to cards selected in draft

            player.has_drafted = true;
            println!("Player [{:?}] has selected their draft.", player.player_id);
        } else {
            panic!("action should have been draft")
        }
    }
}
