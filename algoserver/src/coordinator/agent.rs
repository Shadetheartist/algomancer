use std::fmt::{Display, Formatter};
use rand::{RngCore};

#[derive(Debug)]
pub struct Agent {
    pub id: AgentId,
    pub key: AgentKey,
    pub username: String
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AgentId(pub u64);

impl Display for AgentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl From<u64> for AgentId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AgentKey(pub u64);

impl Display for AgentKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl From<u64> for AgentKey {
    fn from(value: u64) -> Self {
        Self(value)
    }
}