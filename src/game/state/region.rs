use serde::{Deserialize, Serialize};
use crate::game::state::permanent::Permanent;

use crate::game::state::player::{Player, PlayerId};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct RegionId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Region {
    pub region_id: RegionId,
    pub owner_player_id: PlayerId,
    pub players: Vec<Player>,
    pub permanents: Vec<Permanent>,

}

impl Region {

}
