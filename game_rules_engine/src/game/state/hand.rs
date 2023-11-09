use serde::{Deserialize, Serialize};

use crate::game::state::card::Card;

// a hand is a collection of cards without an order
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::new()
        }
    }
}

