use serde::{Deserialize, Serialize};

use crate::game::state::player::Player;
use crate::game::state::State;

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug, Copy)]
pub struct TeamId(pub u8);

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Team {
    pub id: TeamId,
    // maybe this is computed from each player's values
    pub passed_priority: bool,
    pub has_priority: bool,
    pub has_initiative: bool,
}

impl State {
    pub fn living_players_in_team(&self, team_id: TeamId) -> Vec<&Player> {
        self.players.iter().filter(|p| p.team_id == team_id && p.is_alive).collect()
    }

    pub fn players_in_team(&self, team_id: TeamId) -> Vec<&Player> {
        self.players.iter().filter(|p| p.team_id == team_id).collect()
    }

    pub fn players_in_team_mut(&mut self, team_id: TeamId) -> Vec<&mut Player> {
        self.players.iter_mut().filter(|p| p.team_id == team_id).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, GameOptions};
    use crate::game::state::GameMode;
    use crate::game::state::rng::AlgomancerRngSeed;
    use crate::game::state::team::TeamId;

    #[test]
    fn test_players_in_team() {
        // testing two player
        let game = Game::new(&GameOptions {
            seed: AlgomancerRngSeed::default(),
            game_mode: GameMode::new_player_mode(),
        }).expect("a game object");

        let players = game.state.players_in_team(TeamId(1));
        assert_eq!(game.state.players_in_team(TeamId(1)).iter().count(), 1);
        assert_eq!(game.state.players_in_team(TeamId(2)).iter().count(), 1);

    }
}