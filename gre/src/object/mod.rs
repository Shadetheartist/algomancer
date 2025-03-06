use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::card::Card;
use crate::permanent::Permanent;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
#[derive(Default)]
pub struct ObjectId(pub usize);


impl Display for ObjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("O_{}", self.0))
    }
}



#[derive(Clone, Hash, PartialEq, Eq)]
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


