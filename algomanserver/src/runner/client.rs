use std::fmt::{Display, Formatter};
use rand::{RngCore, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub struct ClientKey(pub u64);

impl Display for ClientKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl From<u64> for ClientKey {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl ClientKey {
    pub fn random() -> Self {
        Self(thread_rng().next_u64())
    }
}