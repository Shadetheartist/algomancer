use serde::{Deserialize, Serialize};
use crate::game::state::faction::FactionAffinity;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cost {
    pub faction_affinities: Vec<FactionAffinity>,
    pub additional_cost: u8,
}

impl Cost {
    pub fn free() -> Cost {
        Cost {
            faction_affinities: Vec::new(),
            additional_cost: 0
        }
    }
}