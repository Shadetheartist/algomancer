use std::fmt::{Display, Formatter};
use std::hash::{Hash};

use serde::{Deserialize, Serialize};
use crate::game::db::{CardPrototypeDatabase, CardPrototypeId};

use crate::game::state::{GameMode, State};
use crate::game::state::card::CardType::Resource;

use crate::game::state::error::{EntityNotFoundError};

use crate::game::state::deck::Deck;

use crate::game::state::player::Player;

use crate::game::state::resource::{ResourceType};
use crate::game::state::unordered_cards::UnorderedCards;

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(tag="timing")]
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
        matches!(self, CardType::Unit(_) | CardType::Spell(_))
    }
}



#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy, Ord, PartialOrd)]
pub struct CardId(pub usize);

impl Display for CardId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Card {
    pub card_id: CardId,
    pub prototype_id: CardPrototypeId,
}


pub enum FindCardResult<'a> {
    InPlayerHand(&'a Player, &'a UnorderedCards, &'a Card),
    InPlayerDiscard(&'a Player, &'a UnorderedCards, &'a Card),
    InPlayerDeck(&'a Player, &'a Deck, &'a Card),
    InPlayerPack(&'a Player, &'a UnorderedCards, &'a Card),
    InCommonDeck(&'a Deck, &'a Card),
}

impl Card {
    pub fn from_resource_type(db: &CardPrototypeDatabase, state: &mut State, resource_type: ResourceType) -> Card {

        let (id, _) = db.prototypes.iter().find(|(_, c)| {
            c.card_type == Resource(resource_type)
        }).expect("a prototype for this resource");

        Self::from_prototype_id(db, state, *id)
    }

    pub fn from_prototype_id(db: &CardPrototypeDatabase, state: &mut State, card_prototype_id: CardPrototypeId) -> Card {
        state.next_card_id += 1;
        let proto = db.prototypes.get(&card_prototype_id).expect("a card prototype in the db");

        Card {
            card_id: CardId(state.next_card_id),
            prototype_id: proto.prototype_id,
        }
    }
}

impl State {

    pub fn find_card(&self, card_id: CardId) -> Result<FindCardResult, EntityNotFoundError> {

        // see if the card is in a player's hand or discard
        for player in self.players() {
            let card = player.hand.iter().find(|c| c.card_id == card_id);
            if let Some(card) = card {
                return Ok(FindCardResult::InPlayerHand(player, &player.hand, card))
            }

            let card = player.discard.iter().find(|c| c.card_id == card_id);
            if let Some(card) = card {
                return Ok(FindCardResult::InPlayerDiscard(player, &player.discard, card))
            }
        }

        // search through decks (either common or constructed)
        match self.game_mode {
            GameMode::LiveDraft { .. } => {
                let deck = self.common_deck.as_ref().expect("a common deck");
                let card = deck.iter().find(|c| c.card_id == card_id);
                if let Some(card) = card {
                    return Ok(FindCardResult::InCommonDeck(deck, card))
                }
            }
            GameMode::Constructed { .. } => {
                for player in self.players() {
                    let deck = &player.own_deck.as_ref().expect("a player's deck");
                    let card = deck.iter().find(|c| c.card_id == card_id);
                    if let Some(card) = card {
                        return Ok(FindCardResult::InPlayerDeck(player, deck, card))
                    }
                }
            },
            _ => todo!(),
        }

        Err(EntityNotFoundError::Card(card_id))
    }
}