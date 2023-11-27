use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use rand::prelude::SliceRandom;

use crate::game::{Game, GameOptions};
use crate::game::db::{CardPrototype, CardPrototypeDatabase, CardPrototypeId};
use crate::game::game_builder::NewGameError::NotSupportedYet;
use crate::game::state::{GameMode, State};
use crate::game::state::card::{Card, CardId,  CardType};
use crate::game::state::card::Timing::{Combat, Default, Haste, Virus};
use crate::game::state::card_collection::CardCollectionId;
use crate::game::state::cost::Cost;
use crate::game::state::deck::Deck;
use crate::game::state::faction::Faction;
use crate::game::state::permanent::Permanent;
use crate::game::state::player::{Player, PlayerId, TeamId};
use crate::game::state::progression::{Phase, PrecombatPhaseStep};
use crate::game::state::region::{Region, RegionId};
use crate::game::state::resource::{ ResourceType};
use crate::game::state::resource::ResourceType::{ManaConverter, Shard};
use crate::game::state::rng::AlgomancerRng;
use crate::game::state::stack::Stack;
use crate::game::state::team_configuration::TeamConfiguration;

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
            game_mode => {
                Err(NotSupportedYet(format!("the game mode [{:?}] is not yet supported", game_mode)))
            }
        }
    }

    fn build_live_draft(options: &GameOptions) -> Result<Game, NewGameError> {



        if let GameMode::LiveDraft { team_configuration, .. } = &options.game_mode {
            let mut algomancer_rng = AlgomancerRng::new(options.seed);

            let mut card_prototypes = HashMap::new();
            let mut id_num = 0;

            fn random_read_card_type(rng: &mut AlgomancerRng)-> CardType {
                let r = rng.gen_range(0..8);

                match r {
                    0 => CardType::Spell(Default),
                    1 => CardType::Spell(Combat),
                    2 => CardType::Spell(Virus),
                    3 => CardType::Unit(Haste),
                    4 => CardType::Unit(Virus),
                    _ => CardType::Unit(Default),
                }
            }

            for i in 0..((10 + 54) * 3) {
                id_num = i + 1;
                let card_prototype_id = CardPrototypeId(id_num);
                card_prototypes.insert(card_prototype_id, CardPrototype {
                    prototype_id: card_prototype_id,
                    name: format!("Card #{}", id_num),
                    text: format!("Text for card #{}.", id_num),
                    costs: Cost::free(),
                    card_type: random_read_card_type(&mut algomancer_rng),
                });
            }

            // add the prototypes for the resource types, mana converters, shards, and, tokens
            id_num += 1;
            let card_prototype_id = CardPrototypeId(id_num);
            let mana_converter_prototype_id = card_prototype_id;
            card_prototypes.insert(card_prototype_id, CardPrototype {
                prototype_id: card_prototype_id,
                name: "Mana Converter".to_string(),
                text: "At the beginning of the mana step, you may exchange this for another resource.".to_string(),
                costs: Cost::free(),
                card_type: CardType::Resource(ManaConverter),
            });

            id_num += 1;
            let card_prototype_id = CardPrototypeId(id_num);
            card_prototypes.insert(card_prototype_id, CardPrototype {
                prototype_id: card_prototype_id,
                name: "Shard".to_string(),
                text: "(Shards add no affinity, but all resources including this can be expended for [1]).".to_string(),
                costs: Cost::free(),
                card_type: CardType::Resource(Shard),
            });

            for f in Faction::all() {
                id_num += 1;
                let card_prototype_id = CardPrototypeId(id_num);
                card_prototypes.insert(card_prototype_id, CardPrototype {
                    prototype_id: card_prototype_id,
                    name: format!("{:?}", f),
                    text: format!("When I enter play, if you have [{:?} {:?} {:?}], take a shard.", f, f, f),
                    costs: Cost::free(),
                    card_type: CardType::Resource(ResourceType::from_faction(f)),
                });
            }

            id_num += 1;
            let card_prototype_id = CardPrototypeId(id_num);
            card_prototypes.insert(card_prototype_id, CardPrototype {
                prototype_id: card_prototype_id,
                name: "Token Unit".to_string(),
                text: "Why, hello there".to_string(),
                costs: Cost::free(),
                card_type: CardType::UnitToken,
            });

            // takes all the non-token, non-resource card prototypes and maps them to card instances
            let mut card_id_counter = 0;
            let mut cards_for_deck: Vec<Card> = card_prototypes.iter()
                .filter(|(_, c)| {
                    match c.card_type {
                        CardType::Resource(_) | CardType::UnitToken | CardType::SpellToken => false,
                        CardType::Unit(_) | CardType::Spell(_) => true,
                    }
                })
                .map(|(_, c)| {
                    card_id_counter += 1;
                    Card {
                        card_id: CardId(card_id_counter),
                        prototype_id: c.prototype_id,
                    }
                })
                .collect();

            let cards_db = CardPrototypeDatabase {
                prototypes: card_prototypes,
            };

            cards_for_deck.shuffle(&mut algomancer_rng);

            let mut deck = Deck::new(CardCollectionId::new_common_deck());
            for c in cards_for_deck {
                deck.add_to_top(c)
            }

            let state = State {
                depth: 0,
                game_mode: options.game_mode.clone(),
                common_deck: Some(deck),
                rand: algomancer_rng,
                regions: Vec::new(),
                initiative_player: PlayerId(1),
                next_permanent_id: 1,
                next_card_id: card_id_counter + 1,
                next_formation_id: 1,
            };

            let mut game = Game {
                cards_db,
                action_history: Vec::new(),
                state,
            };

            match &team_configuration {
                TeamConfiguration::Ffa { num_players } => {
                    add_players_and_regions(&mut game, &[0, *num_players], mana_converter_prototype_id);
                }
                TeamConfiguration::Teams { teams_of_players } => {
                    add_players_and_regions(&mut game, teams_of_players, mana_converter_prototype_id);
                }
            }

            fn add_players_and_regions(game: &mut Game, teams_of_players: &[u8], mana_converter_prototype_id: CardPrototypeId) {
                let interlaced_players = Game::interlace_players(teams_of_players);
                for (seat, &team_id) in interlaced_players.iter().enumerate() {
                    let player_id = PlayerId((seat + 1) as u8);
                    let team_id = TeamId(team_id + 1);
                    let mut player = Player::new(player_id, team_id, None, None);

                    let region_id = RegionId(player_id.0);

                    let mut permanents = Vec::new();

                    for _ in 0..2 {
                        let prototype = &game.cards_db.prototypes[&mana_converter_prototype_id];
                        let permanent = Permanent::from_card_prototype(
                            prototype,
                            player_id,
                            &mut game.state
                        );
                        permanents.push(permanent);

                        // initial resources count to the first turn
                        player.resources_played_this_turn += 1;
                    }

                    let mut region = Region{
                        id: region_id,
                        owner_player_id: player_id,
                        players: vec![player],
                        unformed_permanents: permanents,
                        attacking_formation: None,
                        defending_formation: None,
                        step: Phase::PrecombatPhase(PrecombatPhaseStep::Untap),
                        stack: Stack::default(),
                    };

                    region.step = Phase::PrecombatPhase(PrecombatPhaseStep::Draw);
                    region.stack.push_priority(player_id);

                    game.state.regions.push(region);

                    game.state.player_draw_n_cards(player_id, 14);
                }
            }

            return Ok(game);
        }

        panic!("don't call this if the game mode isn't live draft")
    }

    /// distributes the various players as evenly as possible amongst the seats at the table
    fn interlace_players(teams_of_players: &[u8]) -> Vec<u8> {
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
    fn interlace_evenly(elements: &[(u8, u8)]) -> Vec<u8> {
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
