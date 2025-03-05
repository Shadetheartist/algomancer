use serde::{Deserialize, Serialize};
use crate::object::ObjectId;
use crate::player::PlayerId;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    PlayerCastSpell(PlayerId, ObjectId)
}