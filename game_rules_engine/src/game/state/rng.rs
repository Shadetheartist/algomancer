use rand::{Error, Rng, RngCore, SeedableRng};
use rand::distributions::uniform::{SampleRange, SampleUniform};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct AlgomancerRng(rand_pcg::Mcg128Xsl64);

pub type AlgomancerRngSeed = [u8; 16];

impl RngCore for AlgomancerRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl AlgomancerRng {
    pub fn new(seed: AlgomancerRngSeed) -> AlgomancerRng {
        let rng =  rand_pcg::Mcg128Xsl64::from_seed(seed);
        AlgomancerRng(rng)
    }
    #[allow(dead_code)]
    pub fn gen_range<T, R>(&mut self, range: R) -> T
        where
            T: SampleUniform,
            R: SampleRange<T>
    {
        self.0.gen_range(range)
    }
}
