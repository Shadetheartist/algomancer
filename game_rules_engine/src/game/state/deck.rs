use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use crate::game::state::card::{Card, CardId};
use crate::game::state::card_collection::CardCollectionId;
use crate::game::state::error::{EntityNotFoundError, StateError};


#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Deck {
    pub id: CardCollectionId,
    cards: VecDeque<Card>
}

impl Deck {
    pub fn new(id: CardCollectionId) -> Deck {
        Deck {
            id,
            cards: VecDeque::new()
        }
    }

    pub fn iter(&self) -> Iter<'_, Card> {
        self.cards.iter()
    }

    pub fn draw(&mut self) -> Result<Card, StateError> {
        if self.cards.is_empty() {
            return Err(StateError::Other);
        }
        Ok(self.cards.remove(0).unwrap())
    }

    pub fn insert(&mut self, idx: usize, card: Card) {
        self.cards.insert(idx, card)
    }

    pub fn add_to_bottom(&mut self, card: Card) {
        self.cards.push_back(card)
    }

    pub fn add_to_top(&mut self, card: Card) {
        self.cards.push_back(card)
    }

    pub fn remove(&mut self, card_id: CardId) -> Result<Card, StateError>  {
        let idx = match self.cards.iter().position(|e| e.card_id == card_id) {
            None => return Err(EntityNotFoundError::Card(card_id).into()),
            Some(idx) => idx
        };
        Ok(self.cards.remove(idx).unwrap())
    }
}
