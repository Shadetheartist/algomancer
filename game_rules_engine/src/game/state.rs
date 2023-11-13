use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rng::{AlgomancerRng, AlgomancerRngSeed};
use serde::{Deserialize, Serialize};

use crate::game::state::card_collection::CardCollection;
use crate::game::state::player::TeamId;
use crate::game::state::region::Region;
use crate::game::state::resource::Faction;
use crate::game::state::team_configuration::TeamConfiguration;

pub mod effect;
pub mod card;
pub mod player;
pub mod progression;
pub mod resource;
pub mod rng;
pub mod priority;
pub mod region;
pub mod permanent;
pub mod formation;
pub mod mutation;
pub mod card_collection;
pub mod team_configuration;
pub mod error;

type ObjectId = i32;


// as described in the manual
// aside from 1v1, i've never played any of these lol
#[allow(dead_code)]
impl TeamConfiguration {
    pub fn one_v_one() -> TeamConfiguration {
        TeamConfiguration::Teams { teams_of_players: vec![1, 1] }
    }

    pub fn two_v_two() -> TeamConfiguration {
        TeamConfiguration::Teams { teams_of_players: vec![2, 2] }
    }

    pub fn three_v_three() -> TeamConfiguration {
        TeamConfiguration::Teams { teams_of_players: vec![3, 3] }
    }

    pub fn two_v_one() -> TeamConfiguration {
        TeamConfiguration::Teams { teams_of_players: vec![2, 1] }
    }

    pub fn ffa(num_players: u8) -> TeamConfiguration {
        TeamConfiguration::FFA { num_players  }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
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

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct State {
    pub game_mode: GameMode,
    pub rand: AlgomancerRng,
    pub common_deck: Option<CardCollection>,
    pub regions: Vec<Region>,
    pub initiative_team: TeamId,
    pub next_permanent_id: usize,
    pub next_card_id: usize,
    pub next_formation_id: usize,
}

impl State {

    // this is useful for testing
    #[allow(dead_code)]
    pub fn default() -> State {
        State {
            game_mode: GameMode::new_player_mode(),
            rand: AlgomancerRng::new(AlgomancerRngSeed::default()),
            common_deck: Some(CardCollection::new_common_deck()),
            regions: Vec::new(),
            initiative_team: TeamId(1),
            next_permanent_id: 1,
            next_card_id: 1,
            next_formation_id: 1,
        }
    }

    #[allow(dead_code)]
    pub fn get_hash_string(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("#{:x}", hasher.finish())
    }

}


#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

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

    #[test]
    fn test_rand_hashable() {
        fn hash_it(r: AlgomancerRng) -> u64 {
            // hash it
            let mut hasher = DefaultHasher::new();
            r.hash(&mut hasher);
            hasher.finish()
        }

        // create an rng instance
        let r1 = setup_rand();
        let r2 = setup_rand();
        let r1_hash = hash_it(r1);
        let r2_hash = hash_it(r2);

        // these should still be equal
        assert_eq!(r1_hash, r2_hash);

        // do another rand instance, but this one is different because it's being used once more
        let mut r3 = setup_rand();
        r3.gen_range(0..10);

        // this hash should be different from the others
        let r3_hash = hash_it(r3);

        // so this should not be equal
        assert_ne!(r1_hash, r3_hash);
    }
}
