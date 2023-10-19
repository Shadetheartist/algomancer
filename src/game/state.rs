use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::{Rng, SeedableRng};
use rand_xorshift;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Zone {
    pub health: i32
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Player {
    pub health: i32
}

#[derive(Eq, PartialEq, Clone)]
pub struct AlgomancerRng {
    inner: rand_xorshift::XorShiftRng
}

pub type AlgomancerRngSeed = [u8; 16];

impl AlgomancerRng {
    pub fn new(seed: AlgomancerRngSeed) -> AlgomancerRng {
        let rand =  rand_xorshift::XorShiftRng::from_seed(seed);
        AlgomancerRng {
            inner: rand
        }
    }
    pub fn gen_range<T, R>(&mut self, range: R) -> T
        where
            T: SampleUniform,
            R: SampleRange<T>
    {
        self.inner.gen_range(range)
    }
}

impl Hash for AlgomancerRng {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let str = serde_json::to_vec(&self.inner).expect("serialized rng inner to json");
        state.write(str.as_slice())
    }

    fn hash_slice<H: Hasher>(_data: &[Self], _state: &mut H) where Self: Sized {
        todo!()
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct State {
    pub rand: AlgomancerRng,
    pub step: i32,
    pub players: Vec<Player>
}

impl State {
    pub fn new(seed: AlgomancerRngSeed) -> State {
        State {
            rand: AlgomancerRng::new(seed),
            step: 0,
            players: Vec::new()
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
    use super::{AlgomancerRng, AlgomancerRngSeed};

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
    fn test_rand_deterministic(){


        // create a rng instances
        let mut r1 = setup_rand();
        let mut r2 = setup_rand();

        let min = 0;
        let max = 1000000;

        let r1_val = r1.gen_range(min..max);
        let r2_val = r2.gen_range(min..max);

        // these should still be equal
        assert_eq!(r1_val, r2_val);

        // do one more generation on r2
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