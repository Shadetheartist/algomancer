use std::sync::{Arc};
use tonic::{async_trait, Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use crate::algomancer;

use crate::algomancer::{AgentPublicInfo, ConnectRequest, ConnectResponse, CreateLobbyRequest, CreateLobbyResponse, JoinLobbyRequest, JoinLobbyResponse, LeaveLobbyRequest, LeaveLobbyResponse, ListLobbiesRequest};

#[derive(Debug)]
pub struct CoordinatorService {
    pub inner: Arc<tokio::sync::RwLock<algomanserver::Coordinator>>
}

impl CoordinatorService {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(tokio::sync::RwLock::new(algomanserver::Coordinator::new())),
        }
    }
}


#[async_trait]
impl algomancer::coordinator_server::Coordinator for CoordinatorService {
    async fn register(&self, request: Request<ConnectRequest>) -> Result<Response<ConnectResponse>, Status> {

        let request = request.get_ref();

        let (agent_id, agent_key) = {
            let mut coordinator = self.inner.write().await;
            coordinator.create_new_agent(request.username.as_str())
        };

        let response = Response::new(ConnectResponse {
            agent_id: agent_id.0,
            agent_key: agent_key.0,
        });

        Ok(response)
    }

    async fn create_lobby(&self, request: Request<CreateLobbyRequest>) -> Result<Response<CreateLobbyResponse>, Status> {
        let request = request.get_ref();

        let lobby_id = {
            let mut coordinator = self.inner.write().await;
            match coordinator.create_lobby_with_host(request.agent_key.into()) {
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
        let request = request.get_ref();

        {
            let mut coordinator = self.inner.write().await;
            match coordinator.join_lobby(request.agent_key.into(), request.lobby_id.into()) {
                Ok(_) => {},
                Err(err) => return Err(Status::from_error(Box::new(err))),
            }
        }

        let response = Response::new(JoinLobbyResponse {});

        Ok(response)
    }

    async fn leave_lobby(&self, request: Request<LeaveLobbyRequest>) -> Result<Response<LeaveLobbyResponse>, Status> {
        let request = request.get_ref();

        {
            let mut coordinator = self.inner.write().await;
            match coordinator.leave_current_lobby(request.agent_key.into()) {
                Ok(_) => {},
                Err(err) => return Err(Status::from_error(Box::new(err))),
            }
        }

        let response = Response::new(LeaveLobbyResponse {});

        Ok(response)
    }

    type LobbyListenStream = ReceiverStream<Result<crate::algomancer::LobbyEvent, Status>>;

    async fn lobby_listen(&self, request: Request<algomancer::LobbyListenRequest>) -> Result<Response<Self::LobbyListenStream>, Status> {
        let request = request.get_ref();

        let (tx, rx) = tokio::sync::mpsc::channel(4);

        let mut lobby_events_rx = {
            let coordinator = self.inner.write().await;
            match coordinator.lobby_listen(request.lobby_id.into()) {
                Ok(rx) => rx,
                Err(err) => return Err(Status::from_error(Box::new(err))),
            }
        };

        tokio::spawn(async move {
            loop {
                match lobby_events_rx.recv().await {
                    Ok(event) => {
                        let event = algomancer::LobbyEvent {
                            event_type: format!("{:?}", event.event_type),
                            event_arg: event.event_arg,
                        };

                        println!("received broadcast event, re-sending to listener. {:?}", event);

                        tx.send(Ok(event)).await.unwrap();
                    }
                    Err(_) => {
                        return
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type ListLobbiesStream = ReceiverStream<Result<crate::algomancer::ListLobbiesResponse, Status>>;

    async fn list_lobbies(&self, _: Request<ListLobbiesRequest>) -> Result<Response<Self::ListLobbiesStream>, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        let lobbies: Vec<algomancer::ListLobbiesResponse> = {
            let coordinator = self.inner.write().await;
            coordinator.lobbies().map(|l| algomancer::ListLobbiesResponse {
                lobby_id: l.id.0,
                lobby_name: "".to_string(),
                agents: l.agent_ids.iter().map(|a| AgentPublicInfo {
                    agent_id: a.0,
                    username: coordinator.try_get_agent(*a).unwrap().username.to_owned()
                }).collect(),
            }).collect()
        };

        tokio::spawn(async move {
            for l in lobbies {
                tx.send(Ok(l)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}