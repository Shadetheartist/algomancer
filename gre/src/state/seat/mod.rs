use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::state::player::PlayerId;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TeamId(pub usize);

impl Display for TeamId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("T_{}", self.0))
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub(crate) struct Seat {
    pub(crate) seat_number: usize,
    pub(crate) team_id: TeamId,
    pub(crate) player_id: Option<PlayerId>
}


impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("S_{}", self.seat_number))
    }
}