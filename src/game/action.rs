use crate::game::state::card::{CardId, Hand};
use crate::game::state::player::PlayerId;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Action {
    // resolves the next stack item, if there are no stack items, it passes priority
    // Once both players pass priority consecutively, the game moves to the next step or phase.
    Resolve,

    // a player selects a hand of cards from a draft pack, leaving 10 cards in the pack
    Draft { player_id: PlayerId, hand: Hand },

    // a card is cast
    Cast(CardId),
}
