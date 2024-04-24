use rocket::serde::{Deserialize, Serialize};
use algomanserver::{AgentId, AgentKey};

#[derive(Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegistrationResponse {
    pub agent_id: AgentId,
    pub agent_key: String,
}