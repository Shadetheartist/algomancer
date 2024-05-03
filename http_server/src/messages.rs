use rocket::serde::Serialize;
use rocket::serde::Deserialize;

use crate::models::{AgentModel, LobbyModel, MigrationInfoModel};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    ServerRequest { value: ServerRequest },
    ServerResponse { value: ServerResponse },
    ServerEvent { value: ServerEvent },
    ClientRequest { value: ClientRequest },
    ClientResponse { value: ClientResponse },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerRequest {
    AgentKeyRequest,
    MigrationInfoRequest
}

impl ServerRequest {
    pub fn is_correct_response_type(&self, res: &ClientResponse) -> bool {
        match self {
            ServerRequest::AgentKeyRequest => matches!(res, ClientResponse::AgentKeyResponse {..}),
            ServerRequest::MigrationInfoRequest => matches!(res, ClientResponse::MigrationInfoResponse {..}),
        }
    }
}


#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerResponse {
    StartGameResponse,
    LobbyCreated {
        lobby: LobbyModel
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent {
    AgentJoinedLobby {
        agent: AgentModel,
        lobby: LobbyModel
    },
    AgentLeftLobby {
        agent_id: String,
    },
    NewHost {
        agent_id: String,
    },
    Migrate {
        agent_id: String,
        migration_info: MigrationInfoModel
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientRequest {
    StartGameRequest {
        agent_key: String,
        lobby_id: String
    },
}

impl ClientRequest {
    pub fn is_correct_response_type(&self, res: &ServerResponse) -> bool {
        match self {
            ClientRequest::StartGameRequest { .. } => matches!(res, ServerResponse::StartGameResponse {..}),
        }
    }
}


#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientResponse {
    AgentKeyResponse {
        agent_key: String,
    },
    MigrationInfoResponse {
        info: MigrationInfoModel
    }
}