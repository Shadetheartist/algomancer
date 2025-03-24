use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::database::PaperCardId;
use crate::state::object::{Object, ObjectId};
use crate::state::{State, StateError};
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

impl State {
    pub fn permanent_mut(
        &mut self,
        permanent_id: &PermanentId,
    ) -> Result<&mut Permanent, StateError> {
        let object_id = ObjectId::from(*permanent_id);
        let object = self.object_mut(&object_id)?;

        match object {
            Object::Permanent { permanent } => Ok(permanent),
            _ => panic!("permanent query found an object but it's not a permanent"),
        }
    }

    pub(crate) fn permanents(&self) -> impl Iterator<Item = &Permanent> {
        self.objects.iter().filter_map(|(_, obj)| {
            if let Object::Permanent { permanent, .. } = obj {
                Some(permanent)
            } else {
                None
            }
        })
    }

    pub(crate) fn permanents_mut(&mut self) -> impl Iterator<Item = &mut Permanent> {
        self.objects.iter_mut().filter_map(|(_, obj)| {
            if let Object::Permanent { permanent, .. } = obj {
                Some(permanent)
            } else {
                None
            }
        })
    }
}