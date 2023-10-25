use serde::{Deserialize, Serialize};

use crate::game::state::card::CardId;
use crate::game::state::player::PlayerId;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Pack {
    pub cards: Vec<CardId>
}
