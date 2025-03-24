use serde::{Deserialize, Serialize};
use crate::state::player::PlayerId;

#[derive(Debug, Clone, Hash, Default, Serialize, Deserialize)]
pub struct Priority(pub Option<PlayerId>);

