use rocket::serde::Serialize;
use serde::Deserialize;
use algomanserver::{Coordinator, Lobby, LobbyId};
use crate::models;
use crate::models::AgentModel;


#[derive(Serialize, Deserialize)]
pub struct LobbyModel {
    pub id: LobbyId,
    pub name: String,
    pub capacity: u8,
    pub mode: String,
    pub agents: Vec<AgentModel>
}

impl LobbyModel {
    pub fn from_coordinator_lobby(coordinator: &Coordinator, lobby: &Lobby) -> Self {
        Self {
            id: lobby.id,
            name: lobby.name.clone(),
            capacity: 4,
            mode: lobby.options.game_mode.to_string(),
            agents: lobby.agent_ids.iter().filter(|a| coordinator.get_agent(**a).is_some()).map(|a| {
                let agent = coordinator.get_agent(*a).expect("missing agents already should be filtered out");

                AgentModel {
                    id: agent.id.to_string(),
                    username: agent.username.to_owned(),
                }
            }).collect(),
        }
    }
}