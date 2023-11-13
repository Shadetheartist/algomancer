use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum TeamConfiguration {
    // this can't be generalized to teams of 1 since it also affects how the turn progresses.
    // and includes the introduction of 'intent cards'
    FFA {
        num_players: u8
    },
    Teams {
        // describes the number of players per team
        teams_of_players: Vec<u8>
    },
}
