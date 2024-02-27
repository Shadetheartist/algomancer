use serde::{Deserialize, Serialize};
use crate::Faction;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Cost {
    Standard {
        faction_affinities: Vec<(Faction, u32)>,
        additional_cost: u32,
    },
    X {
        faction_affinities: Vec<(Faction, u32)>,
    }
}

impl Cost {
    pub fn free() -> Cost {
        Cost::Standard {
            faction_affinities: Vec::new(),
            additional_cost: 0
        }
    }
}
