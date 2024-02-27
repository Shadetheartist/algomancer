use serde::{Deserialize, Serialize};
use crate::{ResourceType, Timing};

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CardType {
    Resource(ResourceType),
    UnitToken,
    SpellToken,
    Unit(Timing),
    Spell(Timing),
    Meta(MetaCardType)
}

impl CardType {
    /// is not a resource or token - i.e. 'real'
    pub fn is_real(&self) -> bool {
        matches!(self, CardType::Unit(_) | CardType::Spell(_))
    }
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum MetaCardType {
    Trigger,
    StolenCard
}