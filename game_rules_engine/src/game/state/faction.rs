use serde::{Deserialize, Serialize};
use crate::game::state::resource::ResourceType;

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

    #[allow(dead_code)]
    pub fn from_resource_type(resource_type: ResourceType) -> Option<Faction> {
        match resource_type {
            ResourceType::Fire => Some(Faction::Fire),
            ResourceType::Earth => Some(Faction::Earth),
            ResourceType::Water => Some(Faction::Water),
            ResourceType::Metal => Some(Faction::Metal),
            ResourceType::Wood => Some(Faction::Wood),
            ResourceType::ManaConverter |
            ResourceType::Shard => None
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FactionAffinity {
    pub faction: Faction,
    pub amount: u8
}
