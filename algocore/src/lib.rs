use serde::{Deserialize, Serialize};


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
            ResourceType::Prismite |
            ResourceType::Dormant |
            ResourceType::Shard => None,
        }
    }
}


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


#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CardType {
    Resource(ResourceType),
    UnitToken,
    SpellToken,
    Unit(Timing),
    Spell(Timing),
    Meta(MetaCardType)
}


#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum MetaCardType {
    Trigger,
    StolenCard
}

impl CardType {
    /// is not a resource or token - i.e. 'real'
    pub fn is_real(&self) -> bool {
        matches!(self, CardType::Unit(_) | CardType::Spell(_))
    }
}


#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(tag="timing")]
pub enum Timing {
    Default,
    Haste,
    Battle,
    Virus
}


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


