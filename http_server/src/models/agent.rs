use rocket::serde::{Deserialize, Serialize};
use algomanserver::AgentId;

#[derive(Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    pub username: String,
}