use serde::{Deserialize, Serialize};
use crate::game::state::card::{Deck, Hand};
use crate::game::state::{DeckMode, PlayMode, State};
use crate::wrap_index::wrap_index;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    id: usize,
    seat: usize,
    team_id: usize,

    health: i32,
    hand: Hand,

    // this may not be used, depending on the game mode
    constructed_deck: Deck
}

impl Player {
    pub fn new(id: usize, seat: usize, team_id: usize) -> Player {
        Player {
            id,
            seat,
            team_id,
            health: 30,
            hand: Hand::new(),
            constructed_deck: Deck::new(),
        }
    }

    pub fn get_deck<'a>(&'a self, state: &'a State) -> &Deck {
        match state.deck_mode {
            DeckMode::CommonDeck => {
                &state.common_deck
            }
            DeckMode::PlayerDecks => {
                &self.constructed_deck
            }
        }
    }

    fn next_neighbor<'a, F>(&'a self, state: &'a State, f: F) -> Option<&Player> where F: Fn(usize, usize) -> i32 {
        let num_players = state.players.len();

        // start at 1 because we don't want to start at this player's seat
        for i in 1..num_players {
            let idx = f(self.seat, i);
            let wrapped_idx = wrap_index(num_players, idx).expect("a wrapped idx");
            let neighbor = &state.players[wrapped_idx];

            match state.play_mode {
                PlayMode::FFA => {
                    return Some(neighbor);
                }
                PlayMode::Teams => {
                    // if they're not with me, then they're against me
                    if self.team_id != neighbor.team_id {
                        return Some(neighbor);
                    }
                }
            }
        }

        None
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


#[cfg(test)]
mod tests {
    use crate::game::{Game, GameOptions};
    use crate::game::state::{DeckMode, PlayMode};
    use crate::game::state::rng::AlgomancerRngSeed;

    #[test]
    fn test_neighbors_2p(){
        let options = GameOptions{
            seed: AlgomancerRngSeed::default(),
            num_players: 2,
            play_mode: PlayMode::Teams,
            deck_mode: DeckMode::CommonDeck,
        };

        let game = Game::new(&options);

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
            num_players: 4,
            play_mode: PlayMode::Teams,
            deck_mode: DeckMode::CommonDeck,
        };

        let game = Game::new(&options);

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