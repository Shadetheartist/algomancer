use std::slice::Iter;
use serde::{Deserialize, Serialize};
use crate::game::state::card::{Card, CardId};
use crate::game::state::card_collection::CardCollectionId;
use crate::game::state::error::{EntityNotFoundError, StateError};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UnorderedCards {
    pub id: CardCollectionId,
    cards: Vec<Card>
}

impl UnorderedCards {
    pub fn new(id: CardCollectionId) -> UnorderedCards {
        UnorderedCards {
            id,
            cards: Vec::new()
        }
    }

    pub fn iter(&self) -> Iter<'_, Card> {
        self.cards.iter()
    }

    pub fn add(&mut self, card: Card)  {
        self.cards.push(card)
    }

    pub fn remove(&mut self, card_id: CardId) -> Result<Card, StateError>  {
        let idx = match self.cards.iter().position(|e| e.card_id == card_id) {
            None => return Err(EntityNotFoundError::Card(card_id).into()),
            Some(idx) => idx
        };
        Ok(self.cards.remove(idx))
    }

    pub fn transfer_to(&mut self, receiver: &mut UnorderedCards, card_id: CardId) -> Result<(), StateError>{
        let card = self.remove(card_id)?;
        receiver.add(card);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}
