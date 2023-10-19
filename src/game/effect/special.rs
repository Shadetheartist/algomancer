use serde::{Deserialize, Serialize};
use crate::game::{state, StateMutator};

#[derive(Clone, Serialize, Deserialize)]
pub struct SpecialEffect {
    pub effect_number: i32,
}

impl StateMutator for SpecialEffect {
    fn name(&self) -> &str {
        "Special"
    }

    fn prepare(&self, _: &mut state::State) -> Self {
        SpecialEffect {
            effect_number: self.effect_number
        }
    }

    fn explain(&self) -> String {
        format!("Sets the game step to {}", self.effect_number)
    }

    fn mutate_state(&self, state: &mut state::State) {
        state.step = self.effect_number
    }
}