use serde::{Deserialize, Serialize};

use crate::game::state::card::CardId;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct DeckId(pub usize);

// a deck is a collection of cards in some order
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Deck {
    pub deck_id: DeckId,
    pub cards: Vec<CardId>,
}

impl Deck {
    pub fn new(id: DeckId) -> Deck {
        Deck {
            deck_id: id,
            cards: Vec::new()
        }
    }

    pub fn top_card_id(&self) -> Option<CardId> {
        if self.cards.len() > 0 {
            return Some(self.cards[0]);
        }

        None
    }
}

