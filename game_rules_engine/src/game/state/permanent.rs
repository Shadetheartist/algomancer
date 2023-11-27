use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototype, CardPrototypeDatabase, CardPrototypeId};

use crate::game::state::card::{Card, CardType};
use crate::game::state::player::PlayerId;
use crate::game::state::State;
#[derive(Hash, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy)]
pub struct PermanentId(pub usize);

impl PermanentId {
    pub fn next(state: &mut State) -> PermanentId {
        let next = state.next_permanent_id;
        state.next_permanent_id = next + 1;
        Self(next)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PermanentCommon {
    pub permanent_id: PermanentId,
    pub controller_player_id: PlayerId,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag="type")]
pub enum Permanent {
    Unit {
        common: PermanentCommon,
        card: Card,
        // modifications: SomeModificationType
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
    pub fn from_unit_card(card: Card, controller_player_id: PlayerId, state: &mut State, db: &CardPrototypeDatabase) -> Permanent {
        let proto = db.prototypes.get(&card.prototype_id).expect("a prototype");

        if let CardType::Unit(_) = &proto.card_type {
            Permanent::Unit {
                common: PermanentCommon {
                    permanent_id: PermanentId::next(state),
                    controller_player_id,
                },
                card
            }
        } else {
            panic!("you need to call this only when the card type is some real card, not a token or resource")
        }
    }

    pub fn from_card_prototype(card_prototype: &CardPrototype, controller_player_id: PlayerId, state: &mut State) -> Permanent {
        match card_prototype.card_type {
            CardType::Resource(_) => {
                Permanent::Resource {
                    common: PermanentCommon {
                        permanent_id: PermanentId::next(state),
                        controller_player_id,
                    },
                    card_prototype_id: card_prototype.prototype_id,
                }
            }
            CardType::UnitToken => {
                Permanent::UnitToken {
                    common: PermanentCommon {
                        permanent_id: PermanentId::next(state),
                        controller_player_id,
                    },
                    card_prototype_id: card_prototype.prototype_id,
                }
            }
            CardType::SpellToken => {
                Permanent::SpellToken {
                    common: PermanentCommon {
                        permanent_id: PermanentId::next(state),
                        controller_player_id,
                    },
                    card_prototype_id: card_prototype.prototype_id,
                }
            }
            _ => {
                panic!("you need to call this only when the card type is a token or resource")
            }
        }
    }
}
