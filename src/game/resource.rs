use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Resource {
    Fire,
    Earth,
    Water,
    Metal,
    Wood,
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cost {
    resource: Resource,
    amount: i32
}

pub type Costs = Vec<Cost>;
