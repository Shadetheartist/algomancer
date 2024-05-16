use rocket::serde::Serialize;
use serde::Deserialize;
use algomanserver::{Coordinator, Lobby, LobbyId};

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
            capacity: lobby.capacity,
            mode: lobby.game_options.game_mode.to_string(),
            agents: lobby.agent_ids.iter().filter(|a| coordinator.try_get_agent(**a).is_ok()).map(|a| {
                let agent = coordinator.try_get_agent(*a).expect("missing agents already should be filtered out");

                AgentModel {
                    id: agent.id.to_string(),
                    username: agent.username.to_owned(),
                }
            }).collect(),
        }
    }
}