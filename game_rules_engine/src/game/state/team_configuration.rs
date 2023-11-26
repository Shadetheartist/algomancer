use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TeamConfiguration {
    // this can't be generalized to teams of 1 since it also affects how the turn progresses.
    // and includes the introduction of 'intent cards'
    Ffa {
        num_players: u8
    },
    Teams {
        // describes the number of players per team
        teams_of_players: Vec<u8>
    },
}

impl TeamConfiguration {
    pub fn one_v_one() -> TeamConfiguration {
        TeamConfiguration::Teams { teams_of_players: vec![1, 1] }
    }

    pub fn two_v_two() -> TeamConfiguration {
        TeamConfiguration::Teams { teams_of_players: vec![2, 2] }
    }

    pub fn three_v_three() -> TeamConfiguration {
        TeamConfiguration::Teams { teams_of_players: vec![3, 3] }
    }

    pub fn ffa(num_players: u8) -> TeamConfiguration {
        TeamConfiguration::Ffa { num_players  }
    }
}
