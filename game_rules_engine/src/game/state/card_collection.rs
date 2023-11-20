use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash};

use serde::{Deserialize, Serialize};

use crate::game::state::error::{EntityNotFoundError, StateError};
use crate::game::state::deck::Deck;
use crate::game::state::player::{Player, PlayerId};
use crate::game::state::State;
use crate::game::state::unordered_cards::UnorderedCards;


/// CardCollectionId is a Copy type so to keep the values small there's a short encoding scheme
///
/// p = player
/// g = game
///
/// h = hand
/// d = discard
/// p = pack
/// D = deck
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Copy)]
pub struct CardCollectionId(pub [char; 4]);




impl CardCollectionId {
    pub fn from_string(str: &str) -> CardCollectionId {
        let mut char_array: [char; 4] = Default::default();
        for (i, c) in str.chars().enumerate() {
            char_array[i] = c;
        }
        CardCollectionId(char_array)
    }

    pub fn new_hand(player_id: PlayerId) -> CardCollectionId {
        let id_string = format!("p{:0>2}h", player_id.0);
        CardCollectionId::from_string(id_string.as_str())
    }

    pub fn new_discard(player_id: PlayerId) -> CardCollectionId {
        let id_string = format!("p{:0>2}d", player_id.0);
        CardCollectionId::from_string(id_string.as_str())
    }

    pub fn new_pack(player_id: PlayerId) -> CardCollectionId {
        let id_string = format!("p{:0>2}p", player_id.0);
        CardCollectionId::from_string(id_string.as_str())
    }

    pub fn new_deck(player_id: PlayerId) -> CardCollectionId {
        let id_string = format!("p{:0>2}D", player_id.0);
        CardCollectionId::from_string(id_string.as_str())
    }

    pub fn new_common_deck() -> CardCollectionId {
        CardCollectionId::from_string("g__D")
    }
}

impl Display for CardCollectionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Debug for CardCollectionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = self.0.iter().collect::<String>();
        let string = format!("\"{}\"", string);
        f.write_str(string.as_str())
    }
}

pub enum FindCardCollectionMutResult<'a> {
    CommonDeck(&'a mut Deck),
    PlayerHand(PlayerId, &'a mut UnorderedCards),
    PlayerDiscard(PlayerId, &'a mut UnorderedCards),
    PlayerDeck(PlayerId, &'a mut Deck),
    PlayerPack(PlayerId, &'a mut UnorderedCards),
}

pub enum FindCardCollectionResult<'a> {
    CommonDeck(&'a Deck),
    PlayerHand(&'a Player, &'a UnorderedCards),
    PlayerDiscard(&'a Player, &'a UnorderedCards),
    PlayerDeck(&'a Player, &'a Deck),
    PlayerPack(&'a Player, &'a UnorderedCards),
}

impl State {
    pub fn find_card_collection_mut(&mut self, id: CardCollectionId) -> Result<FindCardCollectionMutResult, StateError> {
        match self.find_card_collection(id)? {
            FindCardCollectionResult::CommonDeck(_) => {
                let collection = self.common_deck.as_mut().unwrap();
                Ok(FindCardCollectionMutResult::CommonDeck(collection))
            }
            FindCardCollectionResult::PlayerHand(player, _) => {
                let player = self.find_player_mut(player.id)?;
                let collection = &mut player.hand;
                Ok(FindCardCollectionMutResult::PlayerHand(player.id, collection))
            }
            FindCardCollectionResult::PlayerDiscard(player, _) => {
                let player = self.find_player_mut(player.id)?;
                let collection = &mut player.discard;
                Ok(FindCardCollectionMutResult::PlayerDiscard(player.id, collection))
            }
            FindCardCollectionResult::PlayerDeck(player, _) => {
                let player = self.find_player_mut(player.id)?;
                let collection = player.own_deck.as_mut().unwrap();
                Ok(FindCardCollectionMutResult::PlayerDeck(player.id, collection))
            }
            FindCardCollectionResult::PlayerPack(player, _) => {
                let player = self.find_player_mut(player.id)?;
                let collection = player.pack.as_mut().unwrap();
                Ok(FindCardCollectionMutResult::PlayerPack(player.id, collection))
            }
        }
    }

    pub fn find_card_collection(&self, id: CardCollectionId) -> Result<FindCardCollectionResult, StateError> {
        // check if it's the common deck
        if let Some(deck) = &self.common_deck {
            if deck.id == id {
                return Ok(FindCardCollectionResult::CommonDeck(deck));
            }
        }

        let players: Vec<&Player> = self.players().collect();

        // check if it's one of the player's hands
        if let Some(player) = players.iter().find(|p| p.hand.id == id) {
            return Ok(FindCardCollectionResult::PlayerHand(player, &player.hand));
        }

        // check if it's one of the player's discards
        if let Some(player) = players.iter().find(|p| p.discard.id == id) {
            return Ok(FindCardCollectionResult::PlayerHand(player, &player.discard));
        }

        // check if it's one of the player's decks
        if let Some(player) = players.iter().find(|p| {
            if let Some(deck) = &p.own_deck {
                return deck.id == id;
            }
            false
        }) {
            return Ok(FindCardCollectionResult::PlayerDeck(player, player.own_deck.as_ref().unwrap()));
        }

        // check if it's one of the player's pack's
        if let Some(player) = players.iter().find(|p| {
            if let Some(pack) = &p.pack {
                return pack.id == id;
            }
            false
        }) {
            return Ok(FindCardCollectionResult::PlayerPack(player, player.pack.as_ref().unwrap()));
        }

        Err(EntityNotFoundError::CardCollection(id).into())
    }
}