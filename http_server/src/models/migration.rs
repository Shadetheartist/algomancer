use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MigrationInfoModel {
    pub runner_id: String,
    pub agent_key: String,
    pub client_key: String,
}