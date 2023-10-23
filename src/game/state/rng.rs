use rand::distributions::uniform::{SampleRange, SampleUniform};
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use rand::{Rng, SeedableRng};

// wrapping some other seedable rng in the inner field,
// so that we can implement the hash trait ourselves
#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct AlgomancerRng {
    pub rng: rand_pcg::Mcg128Xsl64
}

pub type AlgomancerRngSeed = [u8; 16];

impl AlgomancerRng {
    pub fn new(seed: AlgomancerRngSeed) -> AlgomancerRng {
        let rng =  rand_pcg::Mcg128Xsl64::from_seed(seed);
        AlgomancerRng {
            rng
        }
    }
    pub fn gen_range<T, R>(&mut self, range: R) -> T
        where
            T: SampleUniform,
            R: SampleRange<T>
    {
        self.rng.gen_range(range)
    }
}

impl Hash for AlgomancerRng {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let str = serde_json::to_vec(&self.rng).expect("serialized rng inner to json");
        state.write(str.as_slice())
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) where Self: Sized {
        for rng in data {
            let str = serde_json::to_vec(&rng.rng).expect("serialized rng inner to json");
            state.write(str.as_slice())
        }
    }
}
