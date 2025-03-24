use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::state::card::CardId;
use crate::state::{State, StateError};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct LibraryId(pub usize);

impl Display for LibraryId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("L_{}", self.0))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Library {
    pub id: LibraryId,
    pub card_ids: VecDeque<CardId>,
}

impl State {
    pub fn library(&self, library_id: &LibraryId) -> Result<&Library, StateError> {
        self.libraries
            .get(library_id)
            .ok_or(StateError::LibraryDoesNotExist(*library_id))
    }

    pub fn library_mut(&mut self, library_id: &LibraryId) -> Result<&mut Library, StateError> {
        self.libraries
            .get_mut(library_id)
            .ok_or(StateError::LibraryDoesNotExist(*library_id))
    }
}