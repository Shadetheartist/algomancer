use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Effect {
    Draw {
        recipient: Recipient,
        amount: u64
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum Recipient {
    #[default]
    Controller
}

