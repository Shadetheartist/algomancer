pub mod special;

use serde::{Deserialize, Serialize};
use super::{ObjectId, state};
use special::SpecialEffect;

#[derive(Serialize, Deserialize, Clone)]
pub enum Effect {
    RandomDamage { target: ObjectId, min: i32, max: i32, prepared_amount: i32 },
    Damage { target: ObjectId, amount: i32 },
    Heal { target: ObjectId, amount: i32 },
    Special(SpecialEffect),
}

impl Effect {

    pub fn name(&self) -> &str {
        match self {
            Effect::Special(effect) => effect.name(),
            Effect::Damage { .. } => "Damage",
            Effect::RandomDamage { .. } => "Random Damage",
            Effect::Heal { .. } => "Heal"
        }
    }

    pub fn prepare(&self, state: &mut state::State) -> Effect {
        match self {
            Effect::RandomDamage { min, max, target, .. } => {
                let amount = state.rand.gen_range(*min..*max);
                let effect = Effect::Damage { amount, target: *target };
                effect
            },
            _ => self.clone()
        }
    }

    pub fn explain(&self) -> String {
        match self {
            Effect::Special(effect) => effect.explain(),
            Effect::Damage { amount, .. } => format!("Deal {} Damage", amount),
            // design issue - the random value should probably be resolved before this part
            Effect::RandomDamage { prepared_amount, max, min, .. } => format!("Deals Between {} and {} Damage [{}]", min, max, prepared_amount),
            Effect::Heal { amount, .. } => format!("Heal {} Damage", amount),
        }
    }

    pub fn mutate_state(&self, state: &mut state::State) {
        match self {
            Effect::Special(effect) => effect.mutate_state(state),
            Effect::Heal { amount, .. } => state.step -= amount,
            Effect::RandomDamage { prepared_amount, .. } => {
                state.step -= prepared_amount
            },
            Effect::Damage { amount, .. } => state.step += amount,
        }
    }
}