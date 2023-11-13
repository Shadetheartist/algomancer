use serde::{Deserialize, Serialize};
use crate::game::state::card::Card;
use crate::game::state::player::{PlayerId, StateError};
use crate::game::state::State;


#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct CardCollectionId(pub [char; 6]);


impl CardCollectionId {
    pub fn from_string(str: &str) -> CardCollectionId {
        let mut char_array: [char; 6] = Default::default();
        for (i, c) in str.chars().enumerate() {
            char_array[i] = c;
        }
        CardCollectionId(char_array)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct CardCollection {
    pub id: CardCollectionId,
    pub cards: Vec<Card>,
}

/// CardCollectionId is a Copy type so to keep the values small there's a short encoding scheme
///
/// p = player
/// g = game
///
/// h = hand
/// d = discard
/// p = pack
/// D = deck
impl CardCollection {
    pub fn new_hand(player_id: PlayerId) -> CardCollection {
        let id_string = format!("p{:0>2}h", player_id.0);
        CardCollection {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: Vec::new(),
        }
    }

    pub fn new_discard(player_id: PlayerId) -> CardCollection {
        let id_string = format!("p{:0>2}d", player_id.0);
        CardCollection {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: Vec::new(),
        }
    }

    pub fn new_pack(player_id: PlayerId) -> CardCollection {
        let id_string = format!("p{:0>2}p", player_id.0);
        CardCollection {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: Vec::new(),
        }
    }

    pub fn new_deck(player_id: PlayerId) -> CardCollection {
        let id_string = format!("p{:0>2}D", player_id.0);
        CardCollection {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: Vec::new(),
        }
    }

    pub fn new_common_deck() -> CardCollection {
        let id_string = "g__D";
        CardCollection {
            id: CardCollectionId::from_string(id_string),
            cards: Vec::new(),
        }
    }

    pub fn draw(&mut self) -> Result<Card, StateError> {
        if self.cards.len() == 0 {
            return Err(StateError::CannotDrawFromEmptyCollection);
        }

        Ok(self.cards.remove(0))
    }
}

impl State {
    pub fn find_card_collection(&self, id: CardCollectionId) -> Option<&CardCollection> {
        /// check if it's the common deck
        if let Some(deck) = &self.common_deck {
            if deck.id == id {
                return Some(deck);
            }
        }

        /// check if it's one of the player's hands
        if let Some(player) = self.players().iter().find(|p| p.hand.id == id) {
            return Some(&player.hand);
        }

        /// check if it's one of the player's discards
        if let Some(player) = self.players().iter().find(|p| p.discard.id == id) {
            return Some(&player.discard);
        }

        /// check if it's one of the player's decks
        if let Some(player) = self.players().iter().find(|p| {
            if let Some(deck) = &p.player_deck {
                return deck.id == id;
            }
            false
        }) {
            let deck = player.player_deck.as_ref().unwrap();
            return Some(deck);
        }

        None
    }
}