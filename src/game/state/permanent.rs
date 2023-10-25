use serde::{Deserialize, Serialize};

use crate::game::state::card::{CardId, CardPrototype, CardPrototypeId, CardType};
use crate::game::state::player::PlayerId;
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
    pub controller_player_id: PlayerId,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Permanent {
    Unit {
        common: PermanentCommon,
        card_id: CardId,
    },
    Resource {
        common: PermanentCommon,
        card_prototype_id: CardPrototypeId,
    },
    SpellToken {
        common: PermanentCommon,
        card_prototype_id: CardPrototypeId,
    },
    UnitToken {
        common: PermanentCommon,
        card_prototype_id: CardPrototypeId,
    },
}

impl Permanent {
    pub fn from_card_prototype(card_prototype: &CardPrototype, controller_player_id: PlayerId, state: &mut State) -> Permanent {
        match card_prototype.card_type {
            CardType::Resource => {
                Permanent::Resource {
                    common: PermanentCommon {
                        permanent_id: PermanentId::next(state),
                        controller_player_id,
                    },
                    card_prototype_id: card_prototype.prototype_id,
                }
            }
            _ => {
                todo!()
            }
        }
    }
}