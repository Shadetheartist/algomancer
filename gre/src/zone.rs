use serde::{Deserialize, Serialize};
use crate::state::library::LibraryId;
use crate::state::player::PlayerId;

#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum Zone {
    Library(LibraryId),
    Hand(PlayerId),
    Battlefield,
    Graveyard(PlayerId),
    Stack,
    Exile,
}