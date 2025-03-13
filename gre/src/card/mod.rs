use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::database::PaperCardId;
use crate::object::ObjectId;
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

