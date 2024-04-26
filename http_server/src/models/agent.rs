use rocket::serde::{Deserialize, Serialize};
use algomanserver::Agent;

#[derive(Serialize, Deserialize)]
pub struct AgentModel {
    pub id: String,
    pub username: String,
}

impl From<&Agent> for AgentModel {
    fn from(value: &Agent) -> Self {
        Self {
            id: value.id.to_string(),
            username: value.username.clone(),
        }
    }
}