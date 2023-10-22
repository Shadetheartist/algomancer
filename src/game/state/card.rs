use serde::{Deserialize, Serialize};
use crate::game::state::effect::EffectBuilder;
use crate::game::state::resource::Costs;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct CardId(pub usize);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Card {
    id: CardId,
    name: String,
    text: String,
    costs: Costs,
    effects: Vec<EffectBuilder>
}
