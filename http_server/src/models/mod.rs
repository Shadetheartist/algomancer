use rocket::serde::{Deserialize, Serialize};
use algomanserver::{AgentKey, LobbyId};

mod lobby;
mod agent;
mod registration;
mod migration;

pub use lobby::LobbyModel;
pub use agent::AgentModel;
pub use migration::MigrationInfoModel;
pub use registration::{RegistrationRequest, RegistrationResponse};

#[derive(Serialize, Deserialize)]
pub struct AgentKeyRequest {
    pub agent_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct AgentLobbyRequest {
    pub agent_key: AgentKey,
    pub lobby_id: LobbyId,
}
