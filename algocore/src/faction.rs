use serde::{Deserialize, Serialize};
use crate::ResourceType;

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

    pub fn to_char(&self) -> char {
        match self {
            Faction::Fire => 'F',
            Faction::Earth => 'E',
            Faction::Water => 'W',
            Faction::Metal => 'M',
            Faction::Wood => 'W',
        }
    }

    #[allow(dead_code)]
    pub fn from_resource_type(resource_type: ResourceType) -> Option<Faction> {
        match resource_type {
            ResourceType::Fire => Some(Faction::Fire),
            ResourceType::Earth => Some(Faction::Earth),
            ResourceType::Water => Some(Faction::Water),
            ResourceType::Metal => Some(Faction::Metal),
            ResourceType::Wood => Some(Faction::Wood),
            ResourceType::Prismite |
            ResourceType::Dormant |
            ResourceType::Shard => None,
        }
    }
}
