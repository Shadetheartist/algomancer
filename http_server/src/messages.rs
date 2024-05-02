use rocket::serde::Serialize;
use rocket::serde::Deserialize;
use algomanserver::runner::MigrationInfo;
use crate::models::{AgentModel, LobbyModel, MigrationInfoModel};

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
    MigrationInfoRequest
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
    },

    MigrationInfoResponse {
        info: MigrationInfoModel
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsEvent {
    AgentJoinedLobby {
        agent: AgentModel,
        lobby: LobbyModel
    },
    Migrate {
        agent_id: String,
        migration_info: MigrationInfoModel
    },

}
