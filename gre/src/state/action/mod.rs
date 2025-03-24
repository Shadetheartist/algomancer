use serde::{Deserialize, Serialize};

mod error;

pub use error::ActionError;
use crate::state::card::CardId;
use crate::state::player::PlayerId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    PlayerAction {
        player_id: PlayerId,
        action: PlayerAction
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerAction {
    Cast {
        card_id: CardId
    }
}