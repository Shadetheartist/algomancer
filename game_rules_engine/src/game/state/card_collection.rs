use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use crate::game::state::card::{Card, CardId};
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

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum CardCollection {
    Deck { id: CardCollectionId, cards: Vec<Card> },
    Hand { id: CardCollectionId, cards: HashSet<Card> },
    Discard { id: CardCollectionId, cards: HashSet<Card> },
    Pack { id: CardCollectionId, cards: HashSet<Card> },
}

impl Hash for CardCollection {
    fn hash<H: Hasher>(&self, state: &mut H) {

        match self {
            CardCollection::Deck { id, cards } => {
                id.hash(state);
                for c in cards {
                    c.hash(state);
                }
            }
            CardCollection::Hand { id, cards } |
            CardCollection::Discard { id, cards } |
            CardCollection::Pack { id, cards } => {
                id.hash(state);

                let mut entries: Vec<&Card> = cards.iter().collect();
                entries.sort_by_key(|a| a.card_id);
                for c in entries {
                    c.hash(state);
                }
            }
        }


    }
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
        CardCollection::Hand {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: HashSet::new(),
        }
    }

    pub fn new_discard(player_id: PlayerId) -> CardCollection {
        let id_string = format!("p{:0>2}d", player_id.0);
        CardCollection::Discard {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: HashSet::new(),
        }
    }

    pub fn new_pack(player_id: PlayerId) -> CardCollection {
        let id_string = format!("p{:0>2}p", player_id.0);
        CardCollection::Pack {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: HashSet::new(),
        }
    }

    pub fn new_deck(player_id: PlayerId) -> CardCollection {
        let id_string = format!("p{:0>2}D", player_id.0);
        CardCollection::Deck {
            id: CardCollectionId::from_string(id_string.as_str()),
            cards: Vec::new(),
        }
    }

    pub fn new_common_deck() -> CardCollection {
        let id_string = "g__D";
        CardCollection::Deck {
            id: CardCollectionId::from_string(id_string),
            cards: Vec::new(),
        }
    }

    pub fn draw(&mut self) -> Result<Card, StateError> {
        match self {
            CardCollection::Deck { cards, .. } => {
                if cards.len() == 0 {
                    return Err(StateError::CannotDrawFromEmptyCollection);
                }
                Ok(cards.remove(0))
            }
            CardCollection::Hand { .. } |
            CardCollection::Discard { .. } |
            CardCollection::Pack { .. } => {
                Err(StateError::CannotDrawFromUnorderedSet)
            }
        }
    }

    pub fn id(&self) -> CardCollectionId {
        match self {
            CardCollection::Deck { id, .. } |
            CardCollection::Hand { id, .. } |
            CardCollection::Discard { id, .. } |
            CardCollection::Pack { id, .. } => {
                *id
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            CardCollection::Deck { cards, .. } => {
                cards.len()
            }
            CardCollection::Hand { cards, .. } |
            CardCollection::Discard { cards, .. } |
            CardCollection::Pack { cards, .. } => {
                cards.len()
            }
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Card> + 'a> {
        match self {
            CardCollection::Deck { cards, .. } => {
                Box::new(cards.iter())
            }
            CardCollection::Hand { cards, .. } |
            CardCollection::Discard { cards, .. } |
            CardCollection::Pack { cards, .. } => {
                Box::new(cards.iter())
            }
        }
    }

    pub fn transfer_to(&mut self, receiver: &mut CardCollection, card_id: CardId) -> Result<(), StateError>{
        let card = self.remove(card_id)?;
        receiver.add(card);
        Ok(())
    }

    pub fn add(&mut self, card: Card) {
        match self {
            CardCollection::Deck { cards, .. } => {
                cards.push(card);
            }
            CardCollection::Hand { cards, .. } |
            CardCollection::Discard { cards, .. } |
            CardCollection::Pack { cards, .. } => {
                cards.insert(card);
            }
        }
    }

    pub fn remove(&mut self, card_id: CardId) -> Result<Card, StateError> {
        match self {
            CardCollection::Deck { cards, .. } => {
                let idx = cards.iter().position(|c| c.card_id == card_id);
                match idx {
                    None => {
                        Err(StateError::CardNotFound(card_id))
                    }
                    Some(idx) => {
                        Ok(cards.remove(idx))
                    }
                }
            }
            CardCollection::Hand { cards, .. } |
            CardCollection::Discard { cards, .. } |
            CardCollection::Pack { cards, .. } => {
                let card = cards.iter().find(|c| c.card_id == card_id);
                match card {
                    None => {
                        Err(StateError::CardNotFound(card_id))
                    }
                    Some(card) => {
                        let card = cards.take(&card.clone()).unwrap();
                        Ok(card)
                    }
                }
            }
        }
    }
}

impl State {
    pub fn find_card_collection_mut(&mut self, id: CardCollectionId) -> Result<&mut CardCollection, StateError> {

        todo!();

        /// check if it's the common deck
        if let Some(deck) = &mut self.common_deck {
            if deck.id() == id {
                return Ok(deck);
            }
        }

        /// check if it's one of the player's hands
        if let Some(player) = self.players_mut().into_iter().find(|p| p.hand.id() == id) {
            return Ok(&mut player.hand);
        }


        /// check if it's one of the player's discards
        if let Some(player) = self.players_mut().into_iter().find(|p| p.discard.id() == id) {
            return Ok(&mut player.discard);
        }

        /// check if it's one of the player's decks
        if let Some(player) = self.players_mut().into_iter().find(|p| {
            if let Some(deck) = &p.player_deck {
                return deck.id() == id;
            }
            false
        }) {
            let deck = player.player_deck.as_mut().unwrap();
            return Ok(deck);
        }

        Err(StateError::CardCollectionNotFound(id))
    }
}