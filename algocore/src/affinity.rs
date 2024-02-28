use serde::{Deserialize, Serialize};
use crate::{Faction};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Affinity {
    pub faction: Faction,
    pub quantity: u32,
}
