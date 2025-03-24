use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::database::PaperCardId;
use crate::state::object::{Object, ObjectId};
use crate::state::{State, StateError};
use crate::zone::Zone;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardId(ObjectId);

impl Display for CardId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("C_{}", self.0))
    }
}

impl From<ObjectId> for CardId {
    fn from(value: ObjectId) -> Self {
        Self(value)
    }
}

impl From<CardId> for ObjectId {
    fn from(value: CardId) -> Self {
        value.0
    }
}


#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Card {
    pub id: CardId,
    pub card_ref: PaperCardId,
    pub zone: Zone
}

impl State {
    pub fn card_mut(&mut self, card_id: &CardId) -> Result<&mut Card, StateError> {
        let object_id = ObjectId::from(*card_id);
        let object = self.object_mut(&object_id)?;

        match object {
            Object::Card { card } => Ok(card),
            _ => panic!("card query found an object but it's not a card"),
        }
    }

    pub(crate) fn cards_mut(&mut self) -> impl Iterator<Item = &mut Card> {
        self.objects.iter_mut().filter_map(|(_, obj)| {
            if let Object::Card { card, .. } = obj {
                Some(card)
            } else {
                None
            }
        })
    }
}
