use std::error::Error;
use std::fmt;

use rand::prelude::SliceRandom;

use crate::game::{Game, GameOptions};
use crate::game::game_builder::NewGameError::NotSupportedYet;
use crate::game::state::{GameMode, State, TeamConfiguration};
use crate::game::state::card::{Card, CardId, CardsDB};
use crate::game::state::deck::{Deck, DeckId};
use crate::game::state::pack::Pack;
use crate::game::state::player::{Player, PlayerId};
use crate::game::state::team::{Team, TeamId};

#[derive(Debug)]
pub enum NewGameError {
    InvalidConfiguration(&'static str),
    NotSupportedYet(String),
}

impl Error for NewGameError {}

impl fmt::Display for NewGameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error creating a new game")
    }
}

impl Game {
    pub fn new(options: &GameOptions) -> Result<Game, NewGameError> {
        match &options.game_mode {
            GameMode::LiveDraft { team_configuration, .. } => {
                let mut cards = Vec::new();
                for i in 0..(54 * 3 + 50) {
                    let card_id = i + 1;
                    cards.push(Card {
                        card_id: CardId(card_id),
                        name: format!("Card #{}", card_id),
                        text: "No card text".to_string(),
                        costs: vec![],
                        effects: vec![],
                    })
                }
                let cards_db = CardsDB { cards };

                let mut state = State::new(options.seed, options.game_mode.clone());

                let mut deck = Deck::new(DeckId(1));
                for c in &cards_db.cards {
                    deck.cards.push(c.card_id)
                }
                deck.cards.shuffle(&mut state.rand.rng);

                state.common_deck = deck;

                let mut game = Game {
                    effect_history: Vec::new(),
                    cards_db: cards_db,
                    state: state,

                };

                match &team_configuration {
                    TeamConfiguration::FFA { num_players } => {
                        for t in 0..*num_players {
                            let team_id = t + 1;

                            game.state.teams.push(Team {
                                id: TeamId(team_id),
                                passed_priority: false,
                                has_priority: false,
                                has_initiative: false,
                            });

                            let player_id = PlayerId(team_id);
                            let mut player = Player::new(player_id, team_id, TeamId(team_id), Pack {
                                owner: player_id,
                                cards: vec![],
                            });

                            Self::draw_opening_hand(&mut game, &mut player);

                            game.state.players.push(player);
                        }
                    }
                    TeamConfiguration::Teams { players } => {
                        Self::interlace_players(players);
                    }
                }

                Ok(game)
            }
            game_mode @ _ => {
                Err(NotSupportedYet(format!("the game mode [{:?}] is not yet supported", game_mode)))
            }
        }
    }

    /// distributes the various players as evenly as possible amongst the seats at the table
    fn interlace_players(teams_of_players: &Vec<u8>) -> Vec<u8> {
        // fleshes out the vec of u8 into a more formal structure so
        // we don't lose track of what team each player is on during the interlacing process
        let teams: Vec<(u8, u8)> = teams_of_players.iter().enumerate().map(|(i, &t)| (i as u8, t)).collect();
        Self::interlace_evenly(&teams)
    }

    /// distributes the various items as evenly as possible amongst indices of the resulting vector
    /// by minimising the 'gap' for each element, which is calculated as the difference between the
    /// desired and actual positions for the next occurrence of each character.
    /// It selects the character with the smallest gap to place next in the result.
    ///
    /// tuple is `[(team_number, num_players)]`
    fn interlace_evenly(elements: &Vec<(u8, u8)>) -> Vec<u8> {
        let total_count: u8 = elements.iter().map(|&(_, count)| count).sum();
        let mut result = Vec::with_capacity(total_count as usize);
        let mut indices = vec![0; elements.len()];

        for _ in 0..total_count {
            let mut min_gap = f64::INFINITY;
            let mut next_idx = 0;

            for (i, &(_, count)) in elements.iter().enumerate() {
                let gap = (total_count as f64) * (indices[i] as f64 + 1.0) / (count as f64) - result.len() as f64;
                if gap < min_gap {
                    min_gap = gap;
                    next_idx = i;
                }
            }

            let (ch, _) = elements[next_idx];
            result.push(ch);
            indices[next_idx] += 1;
        }

        result.into_iter().collect()
    }


    fn draw_opening_hand(mut game: &mut Game, mut player: &mut Player) {
        for _ in 0..16 {
            State::
            move_card(
                game.state.common_deck.top_card_id().expect("the deck should have a top card"),
                &mut game.state.common_deck.cards,
                &mut player.hand.cards)
                .expect("enough cards to draw the opening hand");
        }
    }
}




#[cfg(test)]
mod tests {
    use crate::game::Game;

    #[test]
    fn test_interlace_players() {

        //1v1
        let teams_of_players = vec![1, 1];
        assert_eq!(Game::interlace_players(&teams_of_players), vec![0, 1]);

        // 2v2
        let teams_of_players = vec![2, 2];
        assert_eq!(Game::interlace_players(&teams_of_players), vec![0, 1, 0, 1]);

        // exotic ooh
        let teams_of_players = vec![6, 5, 2, 9];
        assert_eq!(Game::interlace_players(&teams_of_players), vec![3, 0, 1, 3, 0, 3, 1, 3, 0, 2, 3, 1, 0, 3, 3, 1, 0, 3, 0, 1, 2, 3]);
    }

    #[test]
    fn test_interlace_evenly() {

        //FFA
        let counts = vec![(0, 6)];
        assert_eq!(Game::interlace_evenly(&counts), vec![0, 0, 0, 0, 0, 0]);

        //1v1
        let counts = vec![(0, 1), (1, 1)];
        assert_eq!(Game::interlace_evenly(&counts), vec![0, 1]);

        // 2v2
        let counts = vec![(0, 2), (1, 2)];
        assert_eq!(Game::interlace_evenly(&counts), vec![0, 1, 0, 1]);

        // 3v3
        let counts = vec![(0, 3), (1, 3)];
        assert_eq!(Game::interlace_evenly(&counts), vec![0, 1, 0, 1, 0, 1]);

        // 2v1
        let counts = vec![(0, 2), (1, 1)];
        assert_eq!(Game::interlace_evenly(&counts), vec![0, 0, 1]);

        // 2v2v2
        let counts = vec![(0, 2), (1, 2), (2, 2)];
        assert_eq!(Game::interlace_evenly(&counts), vec![0, 1, 2, 0, 1, 2]);

        // 6v2v1
        let counts = vec![(0, 6), (1, 2), (2, 1)];
        assert_eq!(Game::interlace_evenly(&counts), vec![0, 0, 0, 1, 0, 0, 0, 1, 2]);

        // 4v3v2v9
        let counts = vec![(0, 4), (1, 3), (2, 2), (3, 9)];
        assert_eq!(Game::interlace_evenly(&counts), vec![3, 3, 0, 1, 3, 3, 0, 2, 3, 1, 3, 0, 3, 3, 0, 1, 2, 3]);
    }
}
