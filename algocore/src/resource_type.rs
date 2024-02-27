use crate::Faction;
use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum ResourceType {
    Fire,
    Earth,
    Water,
    Metal,
    Wood,
    Shard,
    Prismite,
    Dormant,
}

impl ResourceType {
    pub fn all() -> Vec<ResourceType>{
        vec![
            ResourceType::Fire,
            ResourceType::Earth,
            ResourceType::Water,
            ResourceType::Metal,
            ResourceType::Wood,
            ResourceType::Shard,
        ]
    }

    pub fn from_faction(faction: Faction) -> ResourceType {
        match faction {
            Faction::Fire => ResourceType::Fire,
            Faction::Earth => ResourceType::Earth,
            Faction::Water => ResourceType::Water,
            Faction::Metal => ResourceType::Metal,
            Faction::Wood => ResourceType::Wood,
        }
    }
}
