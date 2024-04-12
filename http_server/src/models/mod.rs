use rocket::serde::{Deserialize, Serialize};
use algomanserver::{AgentKey, LobbyId};

mod lobby;
mod agent;
mod registration;

pub use lobby::Lobby;
pub use agent::Agent;
pub use registration::{RegistrationRequest, RegistrationResponse};

#[derive(Serialize, Deserialize)]
pub struct AgentKeyRequest {
    pub agent_key: AgentKey,
}

#[derive(Serialize, Deserialize)]
pub struct AgentLobbyRequest {
    pub agent_key: AgentKey,
    pub lobby_id: LobbyId,
}
