use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::state::card::Card;
use crate::state::permanent::Permanent;
use crate::state::{State, StateError};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd, Default, Serialize, Deserialize)]
pub struct ObjectId(pub usize);


impl Display for ObjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("O_{}", self.0))
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Object {
    Effect {

    },
    Card {
        card: Card
    },
    Permanent {
        permanent: Permanent
    },
}


impl State {
    pub fn object(&self, object_id: &ObjectId) -> Result<&Object, StateError> {
        self.objects
            .get(object_id)
            .ok_or(StateError::ObjectDoesNotExist(*object_id))
    }

    pub fn object_mut(&mut self, object_id: &ObjectId) -> Result<&mut Object, StateError> {
        self.objects
            .get_mut(object_id)
            .ok_or(StateError::ObjectDoesNotExist(*object_id))
    }
}