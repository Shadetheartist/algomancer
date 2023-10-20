use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::{Rng, SeedableRng};
use rand_pcg;
use serde::{Deserialize, Serialize};
use rng::{AlgomancerRng, AlgomancerRngSeed};
use crate::game::state::card::Deck;
use crate::game::state::player::Player;
use crate::game::state::progression::Phase;
use crate::game::state::progression::Phase::PrecombatPhase;
use crate::game::state::progression::PrecombatPhaseStep::Untap;

pub mod effect;
mod card;
mod zone;
pub mod player;
pub mod progression;
mod resource;
pub mod rng;

type ObjectId = i32;

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum PlayMode {
    FFA,
    Teams
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum DeckMode {
    CommonDeck,
    PlayerDecks
}

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct State {
    pub play_mode: PlayMode,
    pub deck_mode: DeckMode,
    pub common_deck: Deck,
    pub rand: AlgomancerRng,
    pub players: Vec<Player>,
    pub phase: Phase,

    // using this for testing
    pub funny_number: i32,
}

impl State {
    pub fn new(seed: AlgomancerRngSeed, play_mode: &PlayMode, deck_mode: &DeckMode) -> State {
        State {
            play_mode: play_mode.clone(),
            deck_mode: deck_mode.clone(),
            common_deck: Deck::new(),
            rand: AlgomancerRng::new(seed),
            funny_number: 0,
            players: Vec::new(),
            phase: PrecombatPhase(Untap),
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
    use super::{DeckMode, PlayMode, State};

    // utility function to avoid code duplication
    // creates a pre-defined rng instance
    fn setup_rand() -> AlgomancerRng{
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
    fn test_state_serialization(){
        let mut state = State::new(AlgomancerRngSeed::default(), &PlayMode::Teams, &DeckMode::CommonDeck);
        state.funny_number = 100;

        let serialized = serde_json::to_string(&state).expect("stringified state json");
        let deserialized: State = serde_json::from_str(&serialized.as_str()).expect("deserialized state object");

        assert_eq!(state.get_hash_string(), deserialized.get_hash_string());
        assert_eq!(state, deserialized);
    }


    #[test]
    fn test_rand_deterministic(){

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
        let r2_val = r2.gen_range(min..max);
        let r2_val = r2.gen_range(min..max);

        // so this should not be equal
        assert_ne!(r1_val, r2_val);

    }

    #[test]
    fn test_rand_hashable(){

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
