use thiserror::Error;
use crate::state::StateError;

#[derive(Error, Debug)]
pub enum ActionError {
    #[error("state related error: {0}")]
    StateError(#[from] StateError),
}