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

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Cost {
    faction: Faction,
    amount: i32
}

pub type Costs = Vec<Cost>;
