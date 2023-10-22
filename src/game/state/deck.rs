use serde::{Deserialize, Serialize};
use crate::game::state::card::CardId;
use crate::game::state::effect::EffectBuilder;
use crate::game::state::resource::Costs;
use crate::game::state::State;

// a deck is a collection of cards in some order
#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Deck {
    pub cards: Vec<CardId>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: Vec::new()
        }
    }
}
