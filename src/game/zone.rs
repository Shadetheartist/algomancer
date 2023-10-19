use crate::game::player::Player;


pub enum Region<'a> {
    GlobalZone, // for tokens, the main deck, etc
    PlayerRegion(&'a Player),
}

pub enum Zone<'a> {
    GlobalZone, // for tokens, the main deck, etc
    PlayerHand(&'a Player),
    PlayerDiscard(&'a Player),
}