use serde::{Deserialize, Serialize};

use crate::game::state::State;
use crate::game::state::hand::Hand;
use crate::game::state::pack::Pack;
use crate::game::state::team::TeamId;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct PlayerId(pub u8);

impl PlayerId {
    pub fn get_player(self, state: &mut State) -> Option<&mut Player> {
        state.players.iter_mut().find(|p| p.id == self)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub team_id: TeamId,
    pub seat: u8,
    pub is_alive: bool,
    pub has_drafted: bool,
    pub health: i32,
    pub hand: Hand,
    pub pack: Pack,
    pub passed_priority: bool,
}

impl Player {
    pub fn new(id: PlayerId, seat: u8, team_id: TeamId, pack: Pack) -> Player {
        Player {
            id,
            seat,
            team_id,
            is_alive: true,
            has_drafted: false,
            health: 30,
            hand: Hand::new(),
            pack,
            passed_priority: false,
        }
    }

    fn next_neighbor<'a, F>(&'a self, _: &'a State, _: F) -> Option<&Player> where F: Fn(u8, u8) -> i32 {
        todo!()
    }

    // the clockwise neighbor is the next living opponent with a greater seat index. indexes wrap.
    pub fn clockwise_neighbor<'a>(&'a self, state: &'a State) -> Option<&Player> {
        self.next_neighbor(state, |seat, i| (seat + i) as i32)
    }

    // the clockwise neighbor is the next living opponent with a lesser seat index. indexes wrap.
    pub fn counterclockwise_neighbor<'a>(&'a self, state: &'a State) -> Option<&Player> {
        self.next_neighbor(state, |seat, i| seat as i32 - i as i32)
    }
}

impl State {
    pub fn living_players(self, state: &State) -> Vec<&Player> {
        state.players.iter().filter(|p| p.is_alive).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, GameOptions};
    use crate::game::state::GameMode;
    use crate::game::state::rng::AlgomancerRngSeed;

    #[test]
    fn test_neighbors_2p(){
        let options = GameOptions{
            seed: AlgomancerRngSeed::default(),
            game_mode: GameMode::new_player_mode(),
        };

        let game = Game::new(&options).expect("a game object");

        let p1 = &game.state.players[0];

        assert_eq!(p1.seat, 0);

        let cw_p = p1.clockwise_neighbor(&game.state).expect("clockwise neighbor");

        assert_eq!(cw_p.seat, 1);

        let ccw_p = p1.counterclockwise_neighbor(&game.state).expect("counter-clockwise neighbor");

        assert_eq!(ccw_p.seat, 1);
    }

    #[test]
    fn test_neighbors_4p(){
        let options = GameOptions{
            seed: AlgomancerRngSeed::default(),
            game_mode: GameMode::new_player_mode(),
        };

        let game = Game::new(&options).expect("a game object");

        let p1 = &game.state.players[0];

        assert_eq!(p1.seat, 0);

        let cw_p = p1.clockwise_neighbor(&game.state).expect("clockwise neighbor");

        assert_eq!(cw_p.seat, 1);

        let ccw_p = p1.counterclockwise_neighbor(&game.state).expect("counter-clockwise neighbor");

        assert_eq!(ccw_p.seat, 3);
    }

    #[test]
    fn test_neighbors_4p_opponent_death(){
        // should test when an opponent dies
        todo!()
    }
}