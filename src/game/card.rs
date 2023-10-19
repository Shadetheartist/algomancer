use serde::{Deserialize, Serialize};
use crate::game::effect::EffectBuilder;
use crate::game::resource::Costs;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Card {
    name: String,
    text: String,
    costs: Costs,
    effects: Vec<EffectBuilder>
}

// a hand is a collection of cards without an order
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
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

// a deck is a collection of cards in some order
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: Vec::new()
        }
    }
}
