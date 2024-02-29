use serde::{Deserialize, Serialize};
use crate::{Affinity};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Cost {
    Standard {
        threshold: Vec<Affinity>,
        cost: u32,
    },
    X {
        threshold: Vec<Affinity>,
    }
}

impl Cost {
    pub fn free() -> Cost {
        Cost::Standard {
            threshold: Vec::new(),
            cost: 0
        }
    }
}
