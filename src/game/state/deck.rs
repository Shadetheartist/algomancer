use serde::{Deserialize, Serialize};

use crate::game::state::card::Card;

// a deck is a collection of cards in some order
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: Vec::new()
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        if self.cards.len() > 0 {
            return Some(self.cards.remove(0))
        }

        None
    }
}

