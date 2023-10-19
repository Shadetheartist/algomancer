use crate::game::player::Player;

// zones/regions area really important as a top level entity
// objects can only interact with other objects by which zone they're in,
// with the exception of the main deck, which is not in any zone
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Region<'a> {
    GlobalZone, // for tokens, the main deck, etc
    PlayerRegion(&'a Player),
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Zone<'a> {
    GlobalZone, // for tokens, the main deck, etc
    PlayerHand(&'a Player),
    PlayerDiscard(&'a Player),
}