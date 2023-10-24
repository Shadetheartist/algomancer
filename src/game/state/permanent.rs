use serde::{Deserialize, Serialize};

use crate::game::state::card::CardId;
use crate::game::state::player::PlayerId;
use crate::game::state::region::RegionId;
use crate::game::state::resource::{Faction, Resource};
use crate::game::state::State;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct PermanentId(pub usize);

impl PermanentId {
    pub fn next(state: &mut State) -> PermanentId {
        let next = state.next_permanent_id.clone();
        state.next_permanent_id = next + 1;
        PermanentId(next)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct PermanentCommon {
    pub permanent_id: PermanentId,
    pub owner_player_id: PlayerId,
    pub region_id: RegionId,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Permanent {
    Unit {
        common: PermanentCommon,
        card_id: CardId,
    },
    Resource {
        common: PermanentCommon,
        resource_type: Resource,
        // resources should be cards maybe?
    },
    SpellToken {
        common: PermanentCommon,
    },
    UnitToken {
        common: PermanentCommon,
    },
}

