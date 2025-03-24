use serde::{Deserialize, Serialize};
use crate::state::object::ObjectId;
use crate::state::player::PlayerId;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    PlayerCastSpell(PlayerId, ObjectId)
}