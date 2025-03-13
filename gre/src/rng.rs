use core::num::Wrapping as w;
use core::fmt;
use std::hash::{Hash, Hasher};
use rand_core::{RngCore, SeedableRng, Error, impls, le};
use serde::{Deserialize, Serialize};

/// GreRng is basically just the XORshift from rand_xorshift random number generator with Hash
/// implemented.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GreRng {
    x: w<u32>,
    y: w<u32>,
    z: w<u32>,
    w: w<u32>,
}

impl Default for GreRng {
    fn default() -> Self {
        Self::from_seed([0; 16])
    }
}

impl Hash for GreRng {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
        self.w.hash(state);
    }
}

// Custom Debug implementation that does not expose the internal state
impl fmt::Debug for GreRng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GreRng {{}}")
    }
}

impl RngCore for GreRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w_ = self.w;
        self.w = w_ ^ (w_ >> 19) ^ (t ^ (t >> 8));
        self.w.0
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for GreRng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 4];
        le::read_u32_into(&seed, &mut seed_u32);

        // Xorshift cannot be seeded with 0 and we cannot return an Error, but
        // also do not wish to panic (because a random seed can legitimately be
        // 0); our only option is therefore to use a preset value.
        if seed_u32.iter().all(|&x| x == 0) {
            seed_u32 = [0xBAD_5EED, 0xBAD_5EED, 0xBAD_5EED, 0xBAD_5EED];
        }

        Self {
            x: w(seed_u32[0]),
            y: w(seed_u32[1]),
            z: w(seed_u32[2]),
            w: w(seed_u32[3]),
        }
    }

    fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
        let mut b = [0u8; 16];
        loop {
            rng.try_fill_bytes(&mut b[..])?;
            if !b.iter().all(|&x| x == 0) {
                break;
            }
        }

        Ok(Self {
            x: w(u32::from_le_bytes([b[0], b[1], b[2], b[3]])),
            y: w(u32::from_le_bytes([b[4], b[5], b[6], b[7]])),
            z: w(u32::from_le_bytes([b[8], b[9], b[10], b[11]])),
            w: w(u32::from_le_bytes([b[12], b[13], b[14], b[15]])),
        })
    }
}
