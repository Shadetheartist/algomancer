use serde::{Deserialize, Serialize};

use crate::game::state::card::CardId;
use crate::game::state::player::PlayerId;

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug, Copy)]
pub struct PackId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Pack {
    pub pack_id: PackId,
    pub owner_player_id: PlayerId,
    pub cards: Vec<CardId>
}
