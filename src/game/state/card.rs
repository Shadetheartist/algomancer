use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use crate::game::state::resource::Cost;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CardType {
    Resource,
    UnitToken,
    SpellToken,
    Unit,
    Spell,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy, Ord, PartialOrd)]
pub struct CardPrototypeId(pub usize);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototype {
    pub prototype_id: CardPrototypeId,
    pub name: String,
    pub text: String,
    pub costs: Cost,
    pub card_type: CardType,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct CardId(pub usize);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Card {
    pub card_id: CardId,
    pub prototype_id: CardPrototypeId,
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct CardsDB {
    pub card_prototypes: HashMap<CardPrototypeId, CardPrototype>,
    pub card_instances: Vec<Card>,
}

impl Hash for CardsDB {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut entries: Vec<(&CardPrototypeId, &CardPrototype)> = self.card_prototypes.iter().collect();
        entries.sort_by_key(|a| a.0);
        for (k, v) in entries {
            k.hash(state);
            v.hash(state);
        }

        self.card_instances.hash(state);
    }
}