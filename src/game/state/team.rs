use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Team {
    id: i32,
}


#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum Initiative {
    Initiative,
    NonInitiative
}
