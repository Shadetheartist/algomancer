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
        let str = serde_json::to_string(&self.inner).expect("serialized rng inner to json");


    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) where Self: Sized {
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

