
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use rocket::serde::json::Json;
use rocket::State;
use tokio::sync::RwLock;
use ws::{WebSocket};
use algomanserver::{Coordinator, LobbyId};
use crate::{models, services};
use crate::ws::{ws_close_with_error, ws_lobby_listen, ws_send_json, ws_request_agent_key};
use rocket::futures::{StreamExt};
use crate::error::Error;
use crate::messages::ServerEvent::AgentJoinedLobby;
use crate::messages::{WsMessage, ServerResponse};

#[get("/lobbies")]
pub async fn lobbies(coordinator: &State<Arc<RwLock<Coordinator>>>) -> Json<Vec<models::LobbyModel>> {
    let coordinator = coordinator.read().await;

    let lobby_models = services::coordinator_service::lobbies(coordinator.deref());

    Json(lobby_models)
}

#[post("/register", format = "json", data = "<data>")]
pub async fn register(coordinator: &State<Arc<RwLock<Coordinator>>>, data: Json<models::RegistrationRequest>) -> Result<Json<models::RegistrationResponse>, Error> {
    let mut coordinator = coordinator.write().await;

    let response_model = services::coordinator_service::register(
        coordinator.deref_mut(),
        data.username.as_str()
    ).await?;

    Ok(Json(response_model))
}


#[get("/lobby/create")]
pub async fn lobby_create(ws: WebSocket, coordinator: &State<Arc<RwLock<Coordinator>>>, runners: &State<Arc<RwLock<Vec<algomanserver::Runner>>>>) -> ws::Channel<'static> {
    let coordinator = coordinator.inner().clone();
    let runners = runners.inner().clone();

    ws.channel(move |stream| {
        Box::pin(async move {
            let (mut tx, mut rx) = stream.split();

            let agent_key = match ws_request_agent_key(&mut tx, &mut rx).await {
                Ok(agent_key) => agent_key,
                Err(_) => return Ok(())
            };

            let lobby_id = {
                let mut coordinator = coordinator.write().await;
                match coordinator.create_lobby_with_host(agent_key, "Lobby").await {
                    Ok(lobby_id) => lobby_id,
                    Err(err) => {
                        ws_close_with_error(tx, format!("{}", err)).await;
                        return Ok(());
                    }
                }
            };

            {
                let coordinator = coordinator.write().await;
                let lobby = coordinator.try_get_lobby(lobby_id).expect("a lobby");
                let lobby_model = models::LobbyModel::from_coordinator_lobby(coordinator.deref(), lobby);
                if let Err(err) = ws_send_json(&mut tx, &&WsMessage::ServerResponse {value: ServerResponse::LobbyCreated { lobby: lobby_model }}).await {
                    ws_close_with_error(tx, format!("failed to send json {}", err)).await;
                    return Ok(());
                }
            }

            ws_lobby_listen(runners.clone(), coordinator.clone(), agent_key, lobby_id, tx, rx).await;

            {
                let mut coordinator = coordinator.write().await;
                match coordinator.leave_current_lobby(agent_key).await {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }

            Ok(())
        })
    })
}

#[get("/lobby/<lobby_id>/join")]
pub async fn lobby_join(ws: WebSocket, coordinator: &State<Arc<RwLock<Coordinator>>>, runners: &State<Arc<RwLock<Vec<algomanserver::Runner>>>>, lobby_id: u64) -> ws::Channel<'static> {
    let coordinator = coordinator.inner().clone();
    let runners = runners.inner().clone();

    let lobby_id = LobbyId(lobby_id);

    ws.channel(move |stream| {
        Box::pin(async move {
            let (mut tx, mut rx) = stream.split();

            let agent_key = match ws_request_agent_key(&mut tx, &mut rx).await {
                Ok(agent_key) => agent_key,
                Err(err) => {
                    eprintln!("{err}");
                    ws_close_with_error(tx, err.to_string()).await;
                    return Ok(())
                }
            };

            // join lobby
            {
                let mut coordinator = coordinator.write().await;
                match coordinator.join_lobby(agent_key, lobby_id).await {
                    Ok(_) => {}
                    Err(err) => {
                        ws_close_with_error(tx, format!("{}", err)).await;
                        return Ok(());
                    }
                }
            }

            // construct response message
            let message = {
                let coordinator = coordinator.read().await;
                let agent: models::AgentModel = coordinator.get_agent_by_key(agent_key).expect("an agent").into();
                let lobby = models::LobbyModel::from_coordinator_lobby(coordinator.deref(), coordinator.try_get_lobby(lobby_id).expect("a lobby"));
                WsMessage::ServerEvent {
                    value: AgentJoinedLobby {
                        agent,
                        lobby,
                    }
                }
            };

            // send response message
            if let Err(err) = ws_send_json(&mut tx, &message).await {
                ws_close_with_error(tx, format!("{}", err)).await;
                return Ok(());
            }

            // start listening to lobby events
            ws_lobby_listen(runners.clone(), coordinator.clone(), agent_key, lobby_id, tx, rx).await;

            // if the agent stops listening to the lobby, they must leave the lobby
            {
                let mut coordinator = coordinator.write().await;
                match coordinator.leave_current_lobby(agent_key).await {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }

            Ok(())
        })
    })
}
