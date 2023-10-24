use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use rand::prelude::SliceRandom;

use crate::game::{Game, GameOptions};
use crate::game::game_builder::NewGameError::NotSupportedYet;
use crate::game::state::{GameMode, State, TeamConfiguration};
use crate::game::state::card::{Card, CardId, CardPrototype, CardPrototypeId, CardsDB, CardType};
use crate::game::state::deck::{Deck, DeckId};
use crate::game::state::permanent::{Permanent, PermanentCommon, PermanentId};
use crate::game::state::player::{Player, PlayerId};
use crate::game::state::progression::{Phase, PrecombatPhaseStep};
use crate::game::state::region::Region;
use crate::game::state::resource::{Faction, Resource};
use crate::game::state::rng::AlgomancerRng;
use crate::game::state::team::TeamId;

#[derive(Debug)]
pub enum NewGameError {
    //InvalidConfiguration(&'static str),
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
            GameMode::LiveDraft { .. } => {
                Self::build_live_draft(options)
            }
            game_mode @ _ => {
                Err(NotSupportedYet(format!("the game mode [{:?}] is not yet supported", game_mode)))
            }
        }
    }

    fn build_live_draft(options: &GameOptions) -> Result<Game, NewGameError> {
        if let GameMode::LiveDraft { team_configuration, .. } = &options.game_mode {
            let mut algomancer_rng = AlgomancerRng::new(options.seed);

            let mut card_prototypes = HashMap::new();
            let mut id_num = 0;
            for i in 0..((10 + 54) * 3) {
                id_num = i + 1;
                let card_prototype_id = CardPrototypeId(id_num);
                card_prototypes.insert(card_prototype_id, CardPrototype {
                    prototype_id: card_prototype_id,
                    name: format!("Card #{}", id_num),
                    text: format!("Text for card #{}.", id_num),
                    costs: Vec::new(),
                    card_type: CardType::Unit,
                });
            }

            // add the prototypes for the resource types, mana converters, shards, and, tokens
            id_num += 1;
            let card_prototype_id = CardPrototypeId(id_num);
            card_prototypes.insert(card_prototype_id, CardPrototype {
                prototype_id: card_prototype_id,
                name: "Mana Converter".to_string(),
                text: "At the beginning of the mana step, you may exchange this for another resource.".to_string(),
                costs: Vec::new(),
                card_type: CardType::Resource,
            });

            id_num += 1;
            let card_prototype_id = CardPrototypeId(id_num);
            card_prototypes.insert(card_prototype_id, CardPrototype {
                prototype_id: card_prototype_id,
                name: "Shard".to_string(),
                text: "(Shards add no affinity, but all resources including this can be expended for [1]).".to_string(),
                costs: Vec::new(),
                card_type: CardType::Resource,
            });

            for f in Faction::all() {
                id_num += 1;
                let card_prototype_id = CardPrototypeId(id_num);
                card_prototypes.insert(card_prototype_id, CardPrototype {
                    prototype_id: card_prototype_id,
                    name: format!("{:?}", f),
                    text: format!("When I enter play, if you have [{:?} {:?} {:?}], take a shard.", f, f, f),
                    costs: Vec::new(),
                    card_type: CardType::Resource,
                });
            }

            id_num += 1;
            let card_prototype_id = CardPrototypeId(id_num);
            card_prototypes.insert(card_prototype_id, CardPrototype {
                prototype_id: card_prototype_id,
                name: "Token Unit".to_string(),
                text: "Why, hello there".to_string(),
                costs: Vec::new(),
                card_type: CardType::UnitToken,
            });

            // takes all the non-token, non-resource card prototypes and maps them to card instances
            let mut card_id_counter = 0;
            let cards_for_deck = card_prototypes.values()
                .filter(|c| {
                    match c.card_type {
                        CardType::Resource | CardType::UnitToken | CardType::SpellToken => false,
                        CardType::Unit | CardType::Spell => true,
                    }
                })
                .map(|c| {
                    card_id_counter += 1;
                    Card {
                        card_id: CardId(card_id_counter),
                        prototype_id: c.prototype_id,
                    }
                })
                .collect();

            let cards_db = CardsDB {
                card_prototypes,
                card_instances: cards_for_deck,
            };

            let mut deck = Deck::new(DeckId(1));
            for c in &cards_db.card_instances {
                deck.cards.push(c.card_id)
            }
            deck.cards.shuffle(&mut algomancer_rng.rng);

            let mut state = State {
                game_mode: options.game_mode.clone(),
                common_deck_id: deck.deck_id,
                rand: algomancer_rng,
                step: Phase::PrecombatPhase(PrecombatPhaseStep::Untap),
                players: Vec::new(),
                teams: Vec::new(),
                decks: vec![deck],
                regions: Vec::new(),
                permanents: Vec::new(),
                packs: Vec::new(),
                next_permanent_id: 1,
            };

            let mut game = Game {
                effect_history: Vec::new(),
                cards_db: cards_db,
                state: state,
            };

            match &team_configuration {
                TeamConfiguration::FFA { num_players } => {
                    add_players_and_regions(&mut game, &vec![0, *num_players]);
                }
                TeamConfiguration::Teams { teams_of_players } => {
                    add_players_and_regions(&mut game, teams_of_players);
                }
            }

            fn add_players_and_regions(game: &mut Game, teams_of_players: &Vec<u8>) {
                let interlaced_players = Game::interlace_players(teams_of_players);
                for (seat, &team_id) in interlaced_players.iter().enumerate() {
                    let player_id = PlayerId((seat + 1) as u8);
                    let mut player = Player::new(player_id, seat as u8, TeamId(team_id), None);

                    Game::draw_opening_hand(game, &mut player);
                    game.state.players.push(player);

                    let region = Region::from_player_id(&player_id);
                    let region_id = region.region_id;
                    game.state.regions.push(region);

                    for _ in 0..2 {
                        let permanent_id = PermanentId::next(&mut game.state);
                        game.state.permanents.push(Permanent::Resource {
                            common: PermanentCommon {
                                permanent_id: permanent_id,
                                owner_player_id: player_id,
                                region_id: region_id,
                            },
                            resource_type: Resource::ManaConverter,
                        })
                    }
                }
            }

            return Ok(game);
        }

        panic!("don't call this if the game mode isn't live draft")
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


    fn draw_opening_hand(game: &mut Game, player: &mut Player) {
        for _ in 0..16 {
            player.draw_card(&mut game.state)
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
