use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use algomancer_gre::game::GameOptions;
use crate::coordinator::agent::AgentId;
use crate::runner::Runner;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct LobbyId(pub u64);

impl Display for LobbyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl From<u64> for LobbyId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Lobby {
    pub id: LobbyId,

    pub runner: Option<Arc<Mutex<Runner>>>,

    pub options: GameOptions,

    pub host_agent_id: AgentId,
    pub agents: Vec<AgentId>,

    pub broadcast: Option<tokio::sync::broadcast::Sender<LobbyEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LobbyEventType {
    AgentJoined,
    AgentLeft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEvent {
    pub event_type: LobbyEventType,
    pub event_arg: String
}
