use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use crate::models::{AgentModel, LobbyModel};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    Request { value: WsRequest },
    Response { value: WsResponse },
    Event { value: WsEvent },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsRequest {
    AgentKeyRequest,
    StartGameRequest {
        agent_key: String,
        lobby_id: String
    },
}


#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsResponse {
    AgentKeyResponse {
        agent_key: String,
    },

    LaunchGameResponse,

    LobbyCreated {
        lobby: LobbyModel
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsEvent {
    AgentJoinedLobby {
        agent: AgentModel,
        lobby: LobbyModel
    },

}
