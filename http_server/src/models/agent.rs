use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub username: String,
}