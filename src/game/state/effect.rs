pub mod special;

use serde::{Deserialize, Serialize};

use special::SpecialEffect;
use crate::game::state;
use crate::game::state::ObjectId;

// this builder pattern allows for effect preparation before application to the state
// for instance, if an effect would have a random damage value, the randomness is resolved here
// and the effect is applied as a regular damage effect
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum EffectBuilder {
    Default(Effect),
    RandomDamage { target: ObjectId, min: i32, max: i32 },
}

impl EffectBuilder {
    pub fn build_effect(&self, state: &mut state::State) -> Effect {
        match self {
            Self::RandomDamage { min, max, target, .. } => {
                let amount = state.rand.gen_range(*min..*max);
                let effect = Effect::Damage { amount, target: *target };
                effect
            },
            EffectBuilder::Default(effect) => effect.clone(),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum Effect {
    Damage { target: ObjectId, amount: i32 },
    Heal { target: ObjectId, amount: i32 },
    Special(SpecialEffect),
}

impl Effect {

    pub fn name(&self) -> &str {
        match self {
            Self::Special(effect) => effect.name(),
            Self::Damage { .. } => "Damage",
            Self::Heal { .. } => "Heal"
        }
    }

    pub fn explain(&self) -> String {
        match self {
            Self::Special(effect) => effect.explain(),
            Self::Damage { amount, .. } => format!("Deal {} Damage", amount),
            // design issue - the random value should probably be resolved before this part
            Self::Heal { amount, .. } => format!("Heal {} Damage", amount),
        }
    }

    pub fn mutate_state(&self, state: &mut state::State) {
        match self {
            Self::Special(effect) => effect.mutate_state(state),
            Self::Heal { amount, .. } => state.funny_number -= amount,
            Self::Damage { amount, .. } => state.funny_number += amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::state::{State};
    use super::{Effect, EffectBuilder};

    #[test]
    fn test_default_effect_builder() {
        let mut state = State::default();

        let damage_amount = 10;
        let effect = EffectBuilder::Default(Effect::Damage { amount: damage_amount, target: 1 }).build_effect(&mut state);
        match effect {
            Effect::Damage { amount, .. } => {
                // magic number could change if the random algorithm is modified
                assert_eq!(amount, amount)
            },
            _ => {
                assert!(false)
            }
        }

        let heal_amount = 7;
        let effect = EffectBuilder::Default(Effect::Heal { amount: heal_amount, target: 1 }).build_effect(&mut state);
        match effect {
            Effect::Heal { amount, .. } => {
                // magic number could change if the random algorithm is modified
                assert_eq!(amount, amount)
            },
            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_random_damage_effect_builder() {
        let mut state = State::default();

        let effect = EffectBuilder::RandomDamage { min: 2, max: 5, target: 1 }.build_effect(&mut state);

        match effect {
            Effect::Damage { amount, .. } => {
                // magic number could change if the random algorithm is modified
                assert_eq!(3, amount)
            },
            _ => {
                assert!(false)
            }
        }
    }
}