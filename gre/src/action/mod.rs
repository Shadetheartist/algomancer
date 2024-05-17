use serde::{Deserialize, Serialize};

mod error;

pub use error::Error;

#[derive(Clone, Serialize, Deserialize)]
pub struct Action {}

