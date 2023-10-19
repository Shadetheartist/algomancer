use serde::{Deserialize, Serialize};
use crate::game::{state};

// this special effect demonstrates how to extract code out of the effects enum
// this pattern could be solidified in a trait down the road
#[derive(Clone, Serialize, Deserialize)]
pub struct SpecialEffect {
    pub effect_number: i32,
}

impl SpecialEffect {
    pub fn name(&self) -> &str {
        "Special"
    }

    pub fn explain(&self) -> String {
        format!("Sets the game step to {}", self.effect_number)
    }

    pub fn mutate_state(&self, state: &mut state::State) {
        state.step = self.effect_number
    }
}