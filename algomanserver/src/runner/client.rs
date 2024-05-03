
use serde::{Deserialize, Serialize};
use algomacros::impl_u64_key_wrapper;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ClientKey(pub u64);
impl_u64_key_wrapper!(ClientKey);
