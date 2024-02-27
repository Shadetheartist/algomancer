use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(tag="timing")]
pub enum Timing {
    Default,
    Haste,
    Battle,
    Virus
}