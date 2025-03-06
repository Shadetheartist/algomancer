use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Serialize, Deserialize)]
pub enum Effect {
    Draw {
        recipient: Recipient,
        amount: u64
    }
}

#[derive(Debug, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum Recipient {
    #[default]
    Controller
}

