mod parser;

use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

use algocore::{*};

#[derive(Debug)]
pub enum DbError {
    IO(std::io::Error),
    Serde(serde_json::Error),
}


#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy, Ord, PartialOrd)]
pub struct CardPrototypeId(pub usize);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototype {
    pub prototype_id: CardPrototypeId,
    pub name: String,
    pub text: String,
    pub costs: Cost,
    pub card_type: CardType,
    pub std_name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CardPrototypeDatabase {
    pub prototypes: HashMap<CardPrototypeId, CardPrototype>,
}

impl CardPrototypeDatabase {
    pub fn resource(&self, resource_type: ResourceType) -> &CardPrototype {
        let prototype = self.prototypes.iter().find(|(_, c)| {
            c.card_type == CardType::Resource(resource_type)
        });

        if let Some(prototype) = prototype {
            prototype.1
        } else {
            panic!("a prototype for this resource type {:?}", resource_type);
        }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let parsed_prototypes = parser::parse_json(json)?;

        Ok(CardPrototypeDatabase {
            prototypes: parsed_prototypes
        })
    }

    pub fn from_path(file_path: &str) -> Result<Self, DbError> {
        match fs::read_to_string(file_path) {
            Ok(file_content) => {
                match Self::from_json(file_content.as_str()) {
                    Ok(db) => {
                        Ok(db)
                    }
                    Err(err) => {
                        Err(DbError::Serde(err))
                    }
                }
            }
            Err(err) => {
                Err(DbError::IO(err))
            }
        }
    }
}