use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Options {
    pub seed: u128
}

impl Default for Options {
    fn default() -> Self {
        Self { seed: 0xBEEB }
    }
}