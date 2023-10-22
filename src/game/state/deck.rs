use serde::{Deserialize, Serialize};
use crate::game::state::card::CardId;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct DeckId(pub usize);

// a deck is a collection of cards in some order
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Deck {
    pub id: DeckId,
    pub cards: Vec<CardId>,
}

impl Deck {
    pub fn new(id: DeckId) -> Deck {
        Deck {
            id,
            cards: Vec::new()
        }
    }
}
