use serde::{Deserialize, Serialize};
use crate::game::state::card::CardId;

// a hand is a collection of cards without an order
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Hand {
    pub cards: Vec<CardId>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::new()
        }
    }
}

