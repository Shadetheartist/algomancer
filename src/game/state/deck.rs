use serde::{Deserialize, Serialize};

use crate::game::state::card::CardId;
use crate::game::state::State;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct DeckId(pub usize);

impl State {
    pub fn get_deck_mut(&mut self, deck_id: DeckId) -> Option<&mut Deck> {
        self.decks.iter_mut().find(|f| f.deck_id == deck_id)
    }
}

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

