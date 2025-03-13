use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::database::PaperCardId;
use crate::object::ObjectId;
use crate::zone::Zone;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermanentId(ObjectId);

impl Display for PermanentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("C_{}", self.0))
    }
}

impl From<ObjectId> for PermanentId {
    fn from(value: ObjectId) -> Self {
        Self(value)
    }
}

impl From<PermanentId> for ObjectId {
    fn from(value: PermanentId) -> Self {
        value.0
    }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permanent {
    pub id: PermanentId,
    pub card_ref: PaperCardId,
    pub zone: Zone,
}
