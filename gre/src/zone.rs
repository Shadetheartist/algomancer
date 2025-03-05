use serde::{Deserialize, Serialize};
use crate::library::LibraryId;
use crate::player::PlayerId;

#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum Zone {
    Library(LibraryId),
    Hand(PlayerId),
    Battlefield,
    Graveyard(PlayerId),
    Stack,
    Exile,
}