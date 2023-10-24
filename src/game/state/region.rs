use serde::{Deserialize, Serialize};

use crate::game::state::player::PlayerId;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct RegionId(pub usize);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Region {
    pub region_id: RegionId,
    pub player_id: PlayerId,
}

impl Region {
    pub fn from_player_id(player_id: &PlayerId) -> Region {
        Region {
            region_id: RegionId(player_id.0 as usize),
            player_id: player_id.clone()
        }
    }
}
