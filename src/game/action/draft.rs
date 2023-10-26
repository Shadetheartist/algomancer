use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::{GameMode, State};

impl Game {
    pub fn apply_draft_action(&self, state: &mut State, action: &Action) {
        if let Action::Draft { player_id, cards_to_keep } = action {
            let mut new_player = state.player(*player_id).expect("a player").clone();

            match state.game_mode {
                GameMode::LiveDraft { .. }  | GameMode::PreDraft{ .. } | GameMode::TeamDraft { .. }  => {}
                GameMode::Constructed { .. } => { todo!() }
            }

            // todo: constructed vs draft (constructed draw and draft are combined)
            // todo: set hand to cards selected in draft

            new_player.has_drafted = true;

            let mut old_player = state.player_mut(*player_id).expect("the player this action is modifying");
            *old_player = new_player;

            println!("Player [{:?}] has selected their draft.", *player_id);
        } else {
            panic!("action should have been draft")
        }
    }
}
