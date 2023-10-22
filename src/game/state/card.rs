use serde::{Deserialize, Serialize};
use crate::game::state::effect::EffectBuilder;
use crate::game::state::resource::Costs;
use crate::game::state::State;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct CardId(pub usize);
impl CardId {
    pub fn get_card(self, state: &State) -> Option<&Card> {
        state.cards.iter().find(|c| c.id == self)
    }

    pub fn must_get_card(self, state: &State) -> &Card {
        self.get_card(state).expect("card existing in state object")
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Card {
    id: CardId,
    name: String,
    text: String,
    costs: Costs,
    effects: Vec<EffectBuilder>
}

#[cfg(test)]
mod tests {
    use crate::game::state::card::{Card, CardId};
    use crate::game::state::State;

    #[test]
    fn test_get_card(){
        let mut state = State::default();
        let min = 1;
        let max = 32;
        for i in min..max {
            state.cards.push(Card {
                id: CardId(i),
                name: "".to_string(),
                text: "".to_string(),
                costs: vec![],
                effects: vec![],
            })
        }

        // shouldn't exist
        let card = CardId(min-1).get_card(&state);
        assert_eq!(card, None);

        let card = CardId(max+1).get_card(&state);
        assert_eq!(card, None);

        let card = CardId(1).get_card(&state);
        assert_ne!(card, None);

        let card = CardId(13).get_card(&state);
        assert_ne!(card, None);
    }

}
