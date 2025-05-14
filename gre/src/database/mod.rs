mod parser;

use std::collections::{HashMap};
use std::fmt::{Display, Formatter};
use std::{fs, io};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::state::ability::Ability;
use crate::card_type::CardType;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PaperCardId(String);

impl PaperCardId {
    pub(crate) fn new<T: Into<String>>(val: T) -> Self {
        Self(val.into())
    }
}

impl Display for PaperCardId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[[{}]]", self.0))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperCard {
    pub id: PaperCardId,
    pub card_type: CardType,
    pub(crate) abilities: Vec<Ability>
}

impl PaperCard {
    pub(crate) fn triggered_abilities(&self) -> impl Iterator<Item = &Ability> {
        self.abilities.iter().filter(|a| matches!(a, Ability::Triggered(_)))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Database {
    pub(crate) cards: HashMap<PaperCardId, PaperCard>,
}


#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("card data not found for ref {0}")]
    CardDataNotFound(PaperCardId),
    #[error("deserialization (json) error: {0}")]
    SerdeJson(serde_json::Error),
    #[error("io error: {0}")]
    IO(io::Error),
}

impl Database {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let parsed_prototypes = parser::parse_json(json)?;

        Ok(Database {
            cards: parsed_prototypes
        })
    }

    pub fn from_path(file_path: &str) -> Result<Self, DatabaseError> {
        match fs::read_to_string(file_path) {
            Ok(file_content) => {
                match Self::from_json(file_content.as_str()) {
                    Ok(db) => {
                        Ok(db)
                    }
                    Err(err) => {
                        Err(DatabaseError::SerdeJson(err))
                    }
                }
            }
            Err(err) => {
                Err(DatabaseError::IO(err))
            }
        }
    }

    pub(crate) fn card_data(&self, card_ref: &PaperCardId) -> Result<&PaperCard, DatabaseError> {
        self.cards.get(card_ref).ok_or(DatabaseError::CardDataNotFound(card_ref.clone()))
    }


}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_card_ref() {
        assert_eq!(PaperCardId("test_card".into()), PaperCardId("test_card".into()));
    }
}