use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::faction::Faction;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum GameMode {
    LiveDraft {
        selected_deck_types: Vec<Faction>,
        team_configuration: TeamConfiguration,
    },
    PreDraft { team_configuration: TeamConfiguration },
    TeamDraft { team_configuration: TeamConfiguration },
    Constructed { team_configuration: TeamConfiguration },
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::LiveDraft {
            selected_deck_types: vec![Faction::Fire, Faction::Metal],
            team_configuration: TeamConfiguration::one_v_one()
        }
    }
}

impl Display for GameMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMode::LiveDraft { selected_deck_types, .. } => {
                write!(f, "Live Draft (")?;

                for (idx, faction) in selected_deck_types.iter().enumerate() {
                    write!(f, "{}", faction.to_char())?;
                    if idx < selected_deck_types.len() - 1 {
                        write!(f, "/")?;
                    }
                }

                write!(f, ")")?;

                Ok(())
            }
            GameMode::PreDraft { .. } => {
                write!(f, "Pre-Draft")
            }
            GameMode::TeamDraft { .. } => {
                write!(f, "TeamD raft")
            }
            GameMode::Constructed { .. } => {
                write!(f, "Constructed")
            }
        }
    }
}

impl GameMode {
    #[allow(dead_code)]
    pub fn new_player_mode() -> GameMode {
        GameMode::LiveDraft {
            team_configuration: TeamConfiguration::one_v_one(),
            selected_deck_types: vec![Faction::Earth, Faction::Wood]
        }
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
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
