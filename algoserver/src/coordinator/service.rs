use std::sync::{Arc};
use tonic::{async_trait, Request, Response, Status};
use tonic::codegen::tokio_stream;
use crate::algomancer;

use crate::algomancer::{ConnectRequest, ConnectResponse, CreateLobbyRequest, CreateLobbyResponse, JoinLobbyRequest, JoinLobbyResponse, LobbyMessage};
use crate::coordinator::{Error, LobbyId};

#[derive(Debug)]
pub struct CoordinatorService {
    pub inner: Arc<tokio::sync::RwLock<crate::coordinator::Coordinator>>
}

impl CoordinatorService {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(tokio::sync::RwLock::new(crate::coordinator::Coordinator::new())),
        }
    }
}


#[async_trait]
impl algomancer::coordinator_server::Coordinator for CoordinatorService {
    async fn register(&self, request: Request<ConnectRequest>) -> Result<Response<ConnectResponse>, Status> {

        let request = request.get_ref();

        let agent_id = {
            let mut coordinator = self.inner.write().await;
            coordinator.create_new_agent(request.username.as_str())
        };

        let response = Response::new(ConnectResponse {
            agent_id: agent_id.0
        });

        Ok(response)
    }

    async fn create_lobby(&self, request: Request<CreateLobbyRequest>) -> Result<Response<CreateLobbyResponse>, Status> {
        let request = request.get_ref();

        let lobby_id = {
            let mut coordinator = self.inner.write().await;
            match coordinator.create_lobby_with_host(request.agent_id.into()) {
                Ok(lobby_id) => lobby_id,
                Err(err) => return Err(Status::from_error(Box::new(err))),
            }
        };

        let response = Response::new(CreateLobbyResponse {
            lobby_id: lobby_id.0
        });

        Ok(response)
    }

    async fn join_lobby(&self, request: Request<JoinLobbyRequest>) -> Result<Response<JoinLobbyResponse>, Status> {
        todo!()
    }

    type LobbyListenStream = tokio_stream::wrappers::ReceiverStream<Result<LobbyMessage, Status>>;

    async fn lobby_listen(&self, request: Request<JoinLobbyRequest>) -> Result<Response<Self::LobbyListenStream>, Status> {
        todo!()
    }
}