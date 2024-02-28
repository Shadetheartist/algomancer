use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use database::{CardPrototype, CardPrototypeDatabase, CardPrototypeId};

use crate::game::state::card::{Card};
use algocore::{CardType};
use crate::game::state::error::EntityNotFoundError;
use crate::game::state::player::{PlayerId};
use crate::game::state::State;


#[derive(Hash, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy)]
pub struct PermanentId(pub usize);

impl Display for PermanentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permanent #{}", self.0)
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
        tapped: bool,
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
                    permanent_id: PermanentId(state.permanent_id_factory.proceed()),
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
                        permanent_id: PermanentId(state.permanent_id_factory.proceed()),
                        controller_player_id,
                    },
                    card_prototype_id: card_prototype.prototype_id,
                    tapped: false,
                }
            }
            CardType::UnitToken => {
                Permanent::UnitToken {
                    common: PermanentCommon {
                        permanent_id: PermanentId(state.permanent_id_factory.proceed()),
                        controller_player_id,
                    },
                    card_prototype_id: card_prototype.prototype_id,
                }
            }
            CardType::SpellToken => {
                Permanent::SpellToken {
                    common: PermanentCommon {
                        permanent_id: PermanentId(state.permanent_id_factory.proceed()),
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

impl State {
    pub fn find_permanent(&self, id: PermanentId) -> Result<&Permanent, EntityNotFoundError> {
        let find_result = self.regions.iter().flat_map(|region| &region.unformed_permanents).find(|p| match p {
            Permanent::Unit { common, .. } |
            Permanent::Resource { common, .. } |
            Permanent::SpellToken { common, .. } |
            Permanent::UnitToken { common, .. } => {
                common.permanent_id == id
            }
        });

        if let Some(permanent) = find_result {
            Ok(&permanent)
        } else {
            Err(EntityNotFoundError::Permanent(id))
        }
    }

    pub fn find_permanent_mut(&mut self, id: PermanentId) -> Result<&mut Permanent, EntityNotFoundError> {
        let find_result = self.regions.iter_mut().flat_map(|region| &mut region.unformed_permanents).find(|p| match p {
            Permanent::Unit { common, .. } |
            Permanent::Resource { common, .. } |
            Permanent::SpellToken { common, .. } |
            Permanent::UnitToken { common, .. } => {
                common.permanent_id == id
            }
        });

        if let Some(permanent) = find_result {
            Ok(permanent)
        } else {
            Err(EntityNotFoundError::Permanent(id))
        }
    }
}
