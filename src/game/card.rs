use crate::game::effect::EffectBuilder;
use crate::game::resource::Costs;

pub struct Card {
    name: String,
    text: String,
    costs: Costs,
    effects: Vec<EffectBuilder>
}

// a hand is a collection of cards without an order
pub struct Hand {
    cards: Vec<Card>,
}

// a deck is a collection of cards in some order
pub struct Deck {
    cards: Vec<Card>,
}
