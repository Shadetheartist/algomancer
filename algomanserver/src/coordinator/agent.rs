use std::fmt::{Display, Formatter};
use std::str::FromStr;
use rand::{RngCore};
use serde::{Deserialize, Serialize};
use algomacros::impl_u64_key_wrapper;

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    pub key: AgentKey,
    pub username: String,
}

impl Agent {
    pub fn new(id: AgentId, username: String) -> Self {
        Self {
            id,
            key: AgentKey(rand::thread_rng().next_u64()),
            username,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq,Serialize, Deserialize, Hash)]
pub struct AgentId(pub u64);
impl_u64_key_wrapper!(AgentId);


#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct AgentKey(pub u64);
impl_u64_key_wrapper!(AgentKey);
