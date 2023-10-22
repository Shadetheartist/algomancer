use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct TeamId(isize);

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Team {
    pub id: TeamId,
    pub passed_priority: bool,
    pub has_priority: bool,
    pub has_initiative: bool,
}
