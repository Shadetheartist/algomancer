use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use algomancer_gre::game::GameOptions;
use crate::coordinator::agent::AgentId;

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

    pub options: GameOptions,

    pub host_agent_id: AgentId,
    pub agents: Vec<AgentId>,

    pub target: HashMap<AgentId, tokio::sync::mpsc::Sender<LobbyEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum LobbyEvent {
    AgentJoined(AgentId),
    AgentLeft(AgentId),
    NewHost(AgentId),

    Migrate(AgentId),
    Whisper(AgentId, AgentId, String)
}
