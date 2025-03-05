use std::collections::{HashMap};
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::ability::Ability;
use crate::card::Card;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CardRef(pub String);

impl Display for CardRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[[{}]]", self.0))
    }
}

#[derive(Debug, Hash, Serialize, Deserialize)]
pub(crate) struct CardData {
    pub(crate) abilities: Vec<Ability>
}

impl CardData {
    pub(crate) fn triggered_abilities(&self) -> impl Iterator<Item = &Ability> {
        self.abilities.iter().filter(|a| matches!(a, Ability::Triggered(_)))
    }
}

pub struct Database {
    pub(crate) cards: HashMap<CardRef, CardData>,
}

impl Default for Database {
    fn default() -> Self {
        Database {
            cards: Default::default(),
        }
    }
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("card data not found for ref {0}")]
    CardDataNotFound(CardRef),
}

impl Database {
    pub(crate) fn card_data(&self, card_ref: &CardRef) -> Result<&CardData, DatabaseError> {
        self.cards.get(card_ref).ok_or(DatabaseError::CardDataNotFound(card_ref.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_card_ref() {
        assert_eq!(CardRef("test_card".into()), CardRef("test_card".into()));
    }
}