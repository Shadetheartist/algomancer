use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum ResourceType {
    Fire,
    Earth,
    Water,
    Metal,
    Wood,
    ManaConverter,
    Shard,
}

impl ResourceType {
    pub fn all() -> Vec<ResourceType>{
        vec![
            ResourceType::Fire,
            ResourceType::Earth,
            ResourceType::Water,
            ResourceType::Metal,
            ResourceType::Wood,
            ResourceType::ManaConverter,
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

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct FactionAffinity {
    pub faction: Faction,
    pub amount: u8
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
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