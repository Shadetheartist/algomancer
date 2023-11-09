use serde::{Deserialize, Serialize};

use crate::game::state::card::Card;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Pack {
    pub cards: Vec<Card>
}
