use serde::{Deserialize, Serialize};
use crate::game::card::{Deck, Hand};
use crate::game::state::{DeckMode, State};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    health: i32,
    hand: Hand,

    // this may not be used, depending on the game mode
    constructed_deck: Deck
}

impl Player {
    pub fn new() -> Player {
        Player {
            health: 30,
            hand: Hand::new(),
            constructed_deck: Deck::new(),
        }
    }

    pub fn get_deck<'a>(&'a self, state: &'a State) -> &Deck {
        match state.deck_mode {
            DeckMode::CommonDeck => {
                &state.common_deck
            }
            DeckMode::PlayerDecks => {
                &self.constructed_deck
            }
        }
    }

    // this should return the left & right neighboring opponents
    // pretend the player list is wrapped into a circle.
    // the player's allies are transparent to them, so only opponents can be 'seen'.
    pub fn neighbors() -> Vec<Player> {
        todo!()
    }
}