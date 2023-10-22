use serde::{Deserialize, Serialize};
use crate::game::state::player::{Player};
use crate::game::state::State;

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug, Copy)]
pub struct TeamId(pub usize);

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
    use crate::game::state::{DeckMode, PlayMode};
    use crate::game::state::rng::AlgomancerRngSeed;
    use crate::game::state::team::TeamId;

    #[test]
    fn test_players_in_team() {
        // testing two player
        let game = Game::new(&GameOptions {
            seed: AlgomancerRngSeed::default(),
            num_players: 2,
            play_mode: PlayMode::Teams,
            deck_mode: DeckMode::CommonDeck,
        }).expect("a game object");

        let players = game.state.players_in_team(TeamId(1));
        assert_eq!(game.state.players_in_team(TeamId(1)).iter().count(), 1);
        assert_eq!(game.state.players_in_team(TeamId(2)).iter().count(), 1);


        let options = &GameOptions {
            seed: AlgomancerRngSeed::default(),
            num_players: 3,
            play_mode: PlayMode::Teams,
            deck_mode: DeckMode::CommonDeck,
        };
        // testing 3 player, shouldn't work
        let game = Game::new(options);
        assert_eq!(game.is_err(), true);

        // testing 4 player
        let game = Game::new(&GameOptions {
            seed: AlgomancerRngSeed::default(),
            num_players: 4,
            play_mode: PlayMode::Teams,
            deck_mode: DeckMode::CommonDeck,
        }).expect("a game object");

        let players = game.state.players_in_team(TeamId(1));
        assert_eq!(game.state.players_in_team(TeamId(1)).iter().count(), 2);
        assert_eq!(game.state.players_in_team(TeamId(2)).iter().count(), 2);


        // testing 5 player
        let options = &GameOptions {
            seed: AlgomancerRngSeed::default(),
            num_players: 5,
            play_mode: PlayMode::Teams,
            deck_mode: DeckMode::CommonDeck,
        };
        // testing 3 player, shouldn't work
        let game = Game::new(options);
        assert_eq!(game.is_err(), true);


        // testing 6 player
        let game = Game::new(&GameOptions {
            seed: AlgomancerRngSeed::default(),
            num_players: 6,
            play_mode: PlayMode::Teams,
            deck_mode: DeckMode::CommonDeck,
        }).expect("a game object");

        let players = game.state.players_in_team(TeamId(1));
        assert_eq!(game.state.players_in_team(TeamId(1)).iter().count(), 3);
        assert_eq!(game.state.players_in_team(TeamId(2)).iter().count(), 3);
    }
}