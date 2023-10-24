use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use rng::{AlgomancerRng, AlgomancerRngSeed};

use crate::game::state::deck::{Deck, DeckId};
use crate::game::state::player::Player;
use crate::game::state::progression::{Phase, PrecombatPhaseStep};
use crate::game::state::resource::Resource;
use crate::game::state::team::Team;

pub mod effect;
pub mod card;
pub mod zone;
pub mod player;
pub mod progression;
pub mod resource;
pub mod rng;
pub mod team;
pub mod priority;
pub mod stack;
pub mod pack;
pub mod deck;
pub mod hand;

type ObjectId = i32;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum TeamConfiguration {
    FFA {
        num_players: u8
    },
    Teams {
        // describes the number of players per team
        players: Vec<u8>
    },
}

// as described in the manual
// aside from 1v1, i've never played any of these lol
impl TeamConfiguration {
    pub fn one_v_one() -> TeamConfiguration {
        TeamConfiguration::Teams { players: vec![1, 1] }
    }

    pub fn three_v_three() -> TeamConfiguration {
        TeamConfiguration::Teams { players: vec![3, 3] }
    }

    pub fn two_v_one() -> TeamConfiguration {
        TeamConfiguration::Teams { players: vec![2, 1] }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum GameMode {
    LiveDraft {
        selected_deck_types: Vec<Resource>,
        team_configuration: TeamConfiguration,
    },
    PreDraft { team_configuration: TeamConfiguration },
    TeamDraft { team_configuration: TeamConfiguration },
    Constructed { team_configuration: TeamConfiguration },
}

impl GameMode {
    pub fn new_player_mode() -> GameMode {
        GameMode::LiveDraft {
            team_configuration: TeamConfiguration::one_v_one(),
            selected_deck_types: vec![Resource::Earth, Resource::Wood]
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct State {
    pub game_mode: GameMode,
    pub common_deck: Deck,
    pub rand: AlgomancerRng,
    pub step: Phase,

    // store data in a generally flat structure, using id's
    // to reference other objects rather than pointers, at least for now
    pub teams: Vec<Team>,
    pub players: Vec<Player>,
    pub decks: Vec<Deck>,

    // using this for testing
    pub funny_number: i32,
}

impl State {
    pub fn new(seed: AlgomancerRngSeed, game_mode: GameMode) -> State {
        State {
            game_mode: game_mode.clone(),
            common_deck: Deck::new(DeckId(1)),
            rand: AlgomancerRng::new(seed),
            step: Phase::PrecombatPhase(PrecombatPhaseStep::Untap),
            players: Vec::new(),
            teams: Vec::new(),
            decks: Vec::new(),

            funny_number: 0,
        }
    }

    // this is useful for testing
    pub fn default() -> State {
        State {
            game_mode: GameMode::new_player_mode(),
            common_deck: Deck::new(DeckId(1)),
            rand: AlgomancerRng::new(AlgomancerRngSeed::default()),
            step: Phase::PrecombatPhase(PrecombatPhaseStep::Untap),
            players: Vec::new(),
            teams: Vec::new(),
            decks: Vec::new(),

            funny_number: 0,
        }
    }

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

    use super::State;

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
    fn test_state_serialization() {
        let mut state = State::default();
        state.funny_number = 100;

        let serialized = serde_json::to_string(&state).expect("stringified state json");
        let deserialized: State = serde_json::from_str(&serialized.as_str()).expect("deserialized state object");

        assert_eq!(state.get_hash_string(), deserialized.get_hash_string());
        assert_eq!(state, deserialized);
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
