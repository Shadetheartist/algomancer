use serde::{Deserialize, Serialize};
use crate::game::state::card::CardId;
use crate::game::state::effect::EffectBuilder;
use crate::game::state::resource::Costs;
use crate::game::state::State;

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

