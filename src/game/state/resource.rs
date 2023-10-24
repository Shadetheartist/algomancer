use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Resource {
    Fire,
    Earth,
    Water,
    Metal,
    Wood,
    ManaConverter,
    Shard
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Faction {
    Fire,
    Earth,
    Water,
    Metal,
    Wood,
}

impl Faction {
    pub fn all() -> Vec<Faction> {
        vec![
            Faction::Fire,
            Faction::Earth,
            Faction::Water,
            Faction::Metal,
            Faction::Wood,
        ]
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct FactionAffinity {
    pub faction: Faction,
    pub amount: u8
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
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