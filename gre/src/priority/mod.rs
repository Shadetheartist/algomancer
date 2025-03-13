use serde::{Deserialize, Serialize};
use crate::player::PlayerId;

#[derive(Debug, Clone, Copy, Hash, Default, Serialize, Deserialize)]
pub struct Priority(pub(crate) Option<PlayerId>);