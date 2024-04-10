use rocket::serde::Serialize;
use serde::Deserialize;
use algomanserver::LobbyId;
use crate::models::Agent;


#[derive(Serialize, Deserialize)]
pub struct Lobby {
    pub id: LobbyId,
    pub name: String,
    pub agents: Vec<Agent>
}