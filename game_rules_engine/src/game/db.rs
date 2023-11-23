use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game::state::card::CardType;
use crate::game::state::card::CardType::Resource;
use crate::game::state::cost::Cost;
use crate::game::state::resource::ResourceType;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy, Ord, PartialOrd)]
pub struct CardPrototypeId(pub usize);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototype {
    pub prototype_id: CardPrototypeId,
    pub name: String,
    pub text: String,
    pub costs: Cost,
    pub card_type: CardType,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototypeDatabase {
    pub prototypes: HashMap<CardPrototypeId, CardPrototype>,
}

impl CardPrototypeDatabase {
    pub fn resource(&self, resource_type: ResourceType) -> &CardPrototype {
        self.prototypes.iter().find(|(_, c)| {
            c.card_type == Resource(resource_type)
        }).expect("a prototype for this resource").1
    }
}


