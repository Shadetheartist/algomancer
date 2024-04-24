use std::num::ParseIntError;
use std::sync::Arc;
use rocket::serde::json::Json;
use rocket::State;
use tokio::sync::RwLock;
use ws::{Message, WebSocket};
use algomanserver::{AgentId, AgentKey, Coordinator, coordinator, LobbyId};
use crate::{Error, models};
use crate::models::{AgentKeyRequest, RegistrationResponse};
use crate::ws_helpers::{ws_close_with_error, ws_lobby_listen, ws_send_text, ws_wait_for};
use rocket::futures::{SinkExt, StreamExt};

#[get("/lobbies")]
pub async fn lobbies(coordinator: &State<Arc<RwLock<Coordinator>>>) -> Json<Vec<models::Lobby>> {
    let coordinator = coordinator.read().await;
    let public_lobby_info: Vec<models::Lobby> = coordinator.lobbies().map(|l| models::Lobby {
        id: l.id,
        name: l.name.clone(),
        capacity: 4,
        mode: l.options.game_mode.to_string(),
        agents: l.agent_ids.iter().filter(|a| coordinator.get_agent(**a).is_some()).map(|a| {
            let agent = coordinator.get_agent(*a).expect("missing agents already should be filtered out");

            models::Agent {
                id: agent.id.to_string(),
                username: agent.username.to_owned(),
            }
        }).collect(),
    }).collect();

    Json(public_lobby_info)
}

#[post("/register", format = "json", data = "<data>")]
pub async fn register(coordinator: &State<Arc<RwLock<Coordinator>>>, data: Json<models::RegistrationRequest>) -> Json<models::RegistrationResponse> {
    let mut coordinator = coordinator.write().await;

    let (agent_id, agent_key) = coordinator.create_new_agent(data.username.as_str()).await;

    println!("registered agent {agent_key}");

    Json(RegistrationResponse {
        agent_id,
        agent_key: agent_key.to_string(),
    })
}


#[get("/lobby/create")]
pub async fn lobby_create(ws: WebSocket, coordinator: &State<Arc<RwLock<Coordinator>>>) -> ws::Channel<'static> {
    let coordinator = coordinator.inner().clone();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let (mut tx, mut rx) = stream.split();

            let agent_key = match ws_wait_for::<AgentKeyRequest>("agent key", &mut tx, &mut rx).await {
                None => return Ok(()),
                Some(model) => model.agent_key
            };

            let agent_key : AgentKey = match agent_key.parse::<u64>() {
                Ok(agent_key) => agent_key.into(),
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

            if ws_send_text(&mut tx, "Created Lobby").await.is_err() {
                return Ok(());
            }

            ws_lobby_listen(coordinator.clone(), agent_key, lobby_id, tx, rx).await;

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
pub async fn lobby_join(ws: WebSocket, coordinator: &State<Arc<RwLock<Coordinator>>>, lobby_id: u64) -> ws::Channel<'static> {
    let coordinator = coordinator.inner().clone();
    let lobby_id = LobbyId(lobby_id);

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let (mut tx, mut rx) = stream.split();

            let agent_key = match ws_wait_for::<AgentKeyRequest>("agent key", &mut tx, &mut rx).await {
                None => return Ok(()),
                Some(model) => model.agent_key
            };

            let agent_key : AgentKey = match agent_key.parse::<u64>() {
                Ok(agent_key) => agent_key.into(),
                Err(_) => return Ok(())
            };

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

            if ws_send_text(&mut tx, "Joined Lobby").await.is_err() {
                return Ok(());
            }

            ws_lobby_listen(coordinator.clone(), agent_key, lobby_id, tx, rx).await;

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
