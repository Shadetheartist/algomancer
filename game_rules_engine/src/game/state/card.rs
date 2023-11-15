use std::collections::HashMap;
use std::hash::{Hash};

use serde::{Deserialize, Serialize};

use crate::game::state::{GameMode, State};
use crate::game::state::card::CardType::Resource;
use crate::game::state::formation::Formation;
use crate::game::state::permanent::Permanent;
use crate::game::state::player::Player;
use crate::game::state::region::Region;
use crate::game::state::resource::{Cost, ResourceType};

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum Timing {
    Default,
    Haste,
    Combat,
    Virus
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CardType {
    Resource(ResourceType),
    UnitToken,
    SpellToken,
    Unit(Timing),
    Spell(Timing),
}

impl CardType {
    /// is not a resource or token - i.e. 'real'
    pub fn is_real(&self) -> bool {
        match self {
            CardType::Unit(_)  |
            CardType::Spell(_) => true,
            _ => false
        }
    }
}


#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy, Ord, PartialOrd)]
pub struct CardPrototypeId(pub usize);

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototype {
    pub prototype_id: CardPrototypeId,
    pub name: String,
    pub text: String,
    pub costs: Cost,
    pub card_type: CardType,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy, Ord, PartialOrd)]
pub struct CardId(pub usize);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Card {
    pub card_id: CardId,
    pub prototype_id: CardPrototypeId,
}

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototypeDatabase {
    pub prototypes: HashMap<CardPrototypeId, CardPrototype>,
}

pub enum FindCardResult<'a> {
    InPlayerHand(&'a Player, &'a Card),
    InPlayerDiscard(&'a Player, &'a Card),
    InPlayerDeck(&'a Player, &'a Card),
    InCommonDeck(&'a Card),
    AsPermanentInRegion(&'a Region, &'a Permanent),
    AsPermanentInFormation(&'a Region, &'a Formation<Permanent>, &'a Permanent),
}

impl Card {
    pub fn from_resource_type(db: &CardPrototypeDatabase, state: &mut State, resource_type: ResourceType) -> Card {

        let (id, _) = db.prototypes.iter().find(|(_, c)| {
            c.card_type == Resource(resource_type)
        }).expect("a prototype for this resource");

        Self::from_prototype(db, state, *id)
    }

    pub fn from_prototype(db: &CardPrototypeDatabase, state: &mut State, card_prototype_id: CardPrototypeId) -> Card {
        state.next_card_id += 1;
        let proto = db.prototypes.get(&card_prototype_id).expect("a card prototype in the db");

        Card {
            card_id: CardId(state.next_card_id),
            prototype_id: proto.prototype_id,
        }
    }
}

impl State {

    pub fn find_card(&self, card_id: CardId) -> Option<FindCardResult> {

        // see if the card is in a player's hand or discard
        for player in self.players() {
            let card = player.hand.iter().find(|c| c.card_id == card_id);
            if let Some(card) = card {
                return Some(FindCardResult::InPlayerHand(player, card))
            }

            let card = player.discard.iter().find(|c| c.card_id == card_id);
            if let Some(card) = card {
                return Some(FindCardResult::InPlayerDiscard(player, card))
            }
        }

        // look for the card on the battlefield
        for region in self.regions.iter() {
            for permanent in region.unformed_permanents.iter() {
                if let Permanent::Unit { card, .. } = permanent {
                    if card.card_id == card_id {
                        return Some(FindCardResult::AsPermanentInRegion(region, permanent))
                    }
                }
            }

            // the card could be in a formation
            for formation in region.formations().iter() {
                for permanent in formation.cells_iter() {
                    if let Permanent::Unit { card, .. } = permanent {
                        if card.card_id == card_id {
                            return Some(FindCardResult::AsPermanentInFormation(region, formation, permanent))
                        }
                    }
                }
            }
        }

        // search through decks (either common or constructed)
        match self.game_mode {
            GameMode::LiveDraft { .. } => {
                let deck = self.common_deck.as_ref().expect("a common deck");
                let card = deck.iter().find(|c| c.card_id == card_id);
                if let Some(card) = card {
                    return Some(FindCardResult::InCommonDeck(card))
                }
            }
            GameMode::Constructed { .. } => {
                for player in self.players() {
                    let deck = &player.player_deck.as_ref().expect("a player's deck");
                    let card = deck.iter().find(|c| c.card_id == card_id);
                    if let Some(card) = card {
                        return Some(FindCardResult::InPlayerDeck(player, card))
                    }
                }
            },
            _ => todo!(),
        }

        None
    }
}