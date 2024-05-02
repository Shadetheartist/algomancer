#[macro_export]
macro_rules! impl_u64_key_wrapper {
    ($t:ident) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($t), self.0)
            }
        }

        impl std::fmt::Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($t), self.0)
            }
        }

        impl From<u64> for $t {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl std::str::FromStr for $t {
            type Err = std::num::ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let parsed: u64 = s.parse()?;
                Ok(parsed.into())
            }
        }

        impl $t {
            pub fn random() -> Self {
                use rand::RngCore;
                Self(rand::thread_rng().next_u64())
            }
        }
    };
}