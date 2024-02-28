use rng::{AlgomancerRng, AlgomancerRngSeed};
use serde::{Deserialize, Serialize};
use crate::game::state::card_collection::CardCollectionId;

use crate::game::state::player::PlayerId;
use crate::game::state::region::Region;
use algocore::Faction;
use crate::game::state::deck::Deck;
use crate::game::state::team_configuration::TeamConfiguration;

pub mod card;
pub mod player;
pub mod progression;
pub mod rng;
pub mod region;
pub mod permanent;
pub mod formation;
pub mod mutation;
pub mod card_collection;
pub mod team_configuration;
pub mod error;
pub mod deck;
pub mod unordered_cards;
pub mod stack;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum GameMode {
    LiveDraft {
        selected_deck_types: Vec<Faction>,
        team_configuration: TeamConfiguration,
    },
    PreDraft { team_configuration: TeamConfiguration },
    TeamDraft { team_configuration: TeamConfiguration },
    Constructed { team_configuration: TeamConfiguration },
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IdFactory(pub usize);

impl IdFactory {
    pub fn peek(&self) -> usize {
        self.0 + 1
    }

    pub fn proceed(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct State {
    pub depth: usize,
    pub game_mode: GameMode,
    pub rand: AlgomancerRng,
    pub common_deck: Option<Deck>,
    pub regions: Vec<Region>,
    pub initiative_player: PlayerId,
    pub permanent_id_factory: IdFactory,
    pub card_id_factory: IdFactory,
    pub formation_id_factory: IdFactory,
}

impl Default for State {
    fn default() -> Self {
        State {
            depth: 0,
            game_mode: GameMode::new_player_mode(),
            rand: AlgomancerRng::new(AlgomancerRngSeed::default()),
            common_deck: Some(Deck::new(CardCollectionId::new_common_deck())),
            regions: Vec::new(),
            initiative_player: PlayerId(1),
            permanent_id_factory: IdFactory(0),
            card_id_factory: IdFactory(0),
            formation_id_factory: IdFactory(0),
        }
    }
}

#[cfg(test)]
mod tests {
    

    use crate::game::state::rng::{AlgomancerRng, AlgomancerRngSeed};

    // utility function to avoid code duplication
    // creates a pre-defined rng instance
    fn setup_rand() -> AlgomancerRng {
        let seed = AlgomancerRngSeed::default();

        // create an rng instance
        let mut r = AlgomancerRng::new(seed);

        // use it a few times to modify the internal data
        let min = 0;
        let max = 100;
        for _ in 0..10 {
            r.gen_range(min..max);
        }

        r
    }

    #[test]
    fn test_rand_deterministic() {

        // create a rng instances
        let mut r1 = setup_rand();
        let mut r2 = setup_rand();

        let min = 0;
        let max = 1000000;
        let n = 10;

        for _ in 0..n {
            let r1_val = r1.gen_range(min..max);
            let r2_val = r2.gen_range(min..max);

            // these should still be equal
            assert_eq!(r1_val, r2_val);
        }

        // one generation to make sure things will be different
        let r1_val = r1.gen_range(min..max);

        // do one more generation on r2 than on r1
        r2.gen_range(min..max);
        let r2_val = r2.gen_range(min..max);

        // so this should not be equal
        assert_ne!(r1_val, r2_val);
    }
}
