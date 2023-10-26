use serde::{Deserialize, Serialize};

use crate::game::state::{State};
use crate::game::state::deck::{Deck};
use crate::game::state::hand::Hand;
use crate::game::state::pack::{Pack};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct TeamId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct PlayerId(pub u8);

impl PlayerId {
    pub fn get_player(self, state: &mut State) -> Option<&mut Player> {
        for r in state.regions.iter_mut() {
            match r.players.iter_mut().find(|p| p.player_id == self) {
                None => {
                    continue
                }
                Some(player) => {
                    return Some(player);
                }
            }
        }

        None
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub player_id: PlayerId,
    pub team_id: TeamId,
    pub pack: Option<Pack>,
    pub player_deck: Option<Deck>,
    pub seat: u8,
    pub is_alive: bool,
    pub has_drafted: bool,
    pub health: i32,
    pub hand: Hand,
    pub passed_priority: bool,
}

impl Player {
    pub fn new(player_id: PlayerId, seat: u8, team_id: TeamId, deck: Option<Deck>, pack: Option<Pack>) -> Player {
        Player {
            player_id,
            seat,
            team_id,
            player_deck: deck,
            is_alive: true,
            has_drafted: false,
            health: 30,
            hand: Hand::new(),
            passed_priority: false,
            pack: pack,
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
    pub fn player(&self, player_id: PlayerId) -> Option<&Player> {
        self.players().into_iter().find(|&p| p.player_id == player_id)
    }

    pub fn player_mut(&mut self, player_id: PlayerId) -> Option<&mut Player> {
        self.players_mut().into_iter().find(|p| p.player_id == player_id)
    }

    pub fn players(&self) -> Vec<&Player> {
        self.regions.iter().fold(Vec::new(), |mut acc, region| {
            acc.extend(&region.players);
            acc
        })
    }

    pub fn players_mut(&mut self) -> Vec<&mut Player> {
        self.regions.iter_mut().fold(Vec::new(), |mut acc, region| {
            acc.extend(&mut region.players);
            acc
        })
    }

    pub fn living_players_in_team(&self, team_id: TeamId) -> Vec<&Player> {
        self.players().into_iter().filter(|p| p.team_id == team_id && p.is_alive).collect()
    }

    pub fn teams(&self) -> Vec<TeamId> {
        self.players().into_iter().fold(Vec::new(), |mut acc, player| {
            // add the team to the list if it's no already there
            if acc.iter().find(|&t_id| *t_id == player.team_id) == None {
                acc.push(player.team_id)
            }
            acc
        })
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