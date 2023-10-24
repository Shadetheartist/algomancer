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
pub struct Cost {
    faction: Faction,
    amount: i32
}

pub type Costs = Vec<Cost>;
