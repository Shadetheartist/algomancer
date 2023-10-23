use serde::{Deserialize, Serialize};
use crate::game::state::effect::EffectBuilder;
use crate::game::state::resource::Costs;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct CardId(pub usize);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Card {
    pub card_id: CardId,
    pub name: String,
    pub text: String,
    pub costs: Costs,
    pub effects: Vec<EffectBuilder>
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct CardsDB {
    pub cards: Vec<Card>
}

impl CardsDB {
    pub fn get_card(&self, card_id: CardId) -> Option<&Card> {
        self.cards.iter().find(|c| c.card_id == card_id)
    }
}