use serde::{Deserialize, Serialize};

use crate::game::state::card::Card;

// a hand is a collection of cards without an order
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Discard {
    pub cards: Vec<Card>,
}

impl Discard {
    pub fn new() -> Discard {
        Discard {
            cards: Vec::new()
        }
    }
}

