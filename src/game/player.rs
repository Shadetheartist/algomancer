use std::cell::RefCell;
use crate::game::card::{Deck, Hand};

pub struct Player {
    hand: Hand,

    // this could be the player's own constructed deck, or the main deck
    deck: RefCell<Deck>
}