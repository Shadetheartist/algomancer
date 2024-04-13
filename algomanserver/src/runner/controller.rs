use std::fmt::{Display, Formatter};
use rand::{RngCore, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub struct ControllerKey(pub u64);

impl Display for ControllerKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl From<u64> for ControllerKey {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl ControllerKey {
    pub fn random() -> Self {
        Self(thread_rng().next_u64())
    }

}