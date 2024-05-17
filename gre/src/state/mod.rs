mod sba;
mod procedure;

use rand_core::SeedableRng;
use crate::action::Action;
use crate::options::Options;
use crate::rng::GreRng;
use crate::action::Error as ActionError;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct State {
    options: Options,
    rng: GreRng,
}

impl State {
    pub fn new(options: Options) -> Self {
        let rng = GreRng::from_seed(options.seed.to_le_bytes());
        Self {
            options: options,
            rng: rng
        }
    }

    pub fn apply_action(&self, action: &Action) -> Result<Self, ActionError> {
        let mut state = self.clone();



        Ok(state)
    }

    pub fn valid_actions() -> Vec<Action> {
        vec![]
    }

    pub fn is_terminal() -> bool {
        false
    }
}
