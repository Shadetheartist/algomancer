use serde::{Deserialize, Serialize};

use crate::game::state::{GameMode, State};
use crate::game::state::deck::{Deck};
use crate::game::state::hand::Hand;
use crate::game::state::pack::{Pack};

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct TeamId(pub u8);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct PlayerId(pub u8);

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
}


impl State {
    pub fn player(&self, player_id: PlayerId) -> Option<&Player> {
        self.players().into_iter().find(|&p| p.player_id == player_id)
    }

    pub fn player_mut(&mut self, player_id: PlayerId) -> Option<&mut Player> {
        self.players_mut().into_iter().find(|p| p.player_id == player_id)
    }

    pub fn player_hand_mut(&mut self, player_id: PlayerId) -> &mut Hand {
        &mut self.player_mut(player_id).expect("a player").hand
    }

    pub fn player_pack_mut(&mut self, player_id: PlayerId) -> Option<&mut Pack> {
        self.player_mut(player_id).expect("a player").pack.as_mut()
    }

    pub fn player_deck_mut(&mut self, player_id: PlayerId) -> &mut Deck {
        match &self.game_mode {
            GameMode::LiveDraft { .. } => {
                if let Some(common_deck) = &mut self.common_deck {
                    common_deck
                } else {
                    panic!("player is supposed to draw from the common deck in live-draft, but it doesn't exist");
                }
            },
            GameMode::PreDraft { .. } | GameMode::Constructed { .. } => {
                let mut player = self.player_mut(player_id).expect("player");
                if let Some(player_deck) = player.player_deck.as_mut() {
                    player_deck
                } else {
                    panic!("player is supposed to draw from their own deck in pre-draft & constructed, but it doesn't exist");
                }
            },
            GameMode::TeamDraft { .. } => {
                // weird, this needs a common deck per team i guess
                todo!("need to implement team draft, which deck the player is drawing from")
            }
        }
    }

    pub fn players(&self) -> Vec<&Player> {
        self.regions.iter().flat_map(|r| &r.players).collect()
    }

    pub fn players_mut(&mut self) -> Vec<&mut Player> {
        self.regions.iter_mut().flat_map(|r| &mut r.players).collect()
    }

    pub fn player_ids(&self) -> Vec<PlayerId> {
        self.regions.iter().flat_map(|r| &r.players).map(|p| p.player_id ).collect()
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