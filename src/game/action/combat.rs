use crate::game::action::Action;
use crate::game::Game;
use crate::game::state::player::StateError;
use crate::game::state::State;

impl Game {
    pub fn apply_attack_action(&self, mut state: State, action: &Action) -> Result<State, StateError> {
        if let Action::PlayCard { card_id } = action {



            Ok(state)
        } else {
            panic!("only call this when the action is of the correct enum type")
        }
    }
}