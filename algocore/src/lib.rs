use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DbError {
    IO(std::io::Error),
    Serde(serde_json::Error),
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Faction {
    Fire,
    Earth,
    Water,
    Metal,
    Wood,
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


#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy, Ord, PartialOrd)]
pub struct CardPrototypeId(pub usize);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototype {
    pub prototype_id: CardPrototypeId,
    pub name: String,
    pub text: String,
    pub costs: Cost,
    pub card_type: CardType,

    pub std_name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototypeDatabase {
    pub prototypes: HashMap<CardPrototypeId, CardPrototype>,
}