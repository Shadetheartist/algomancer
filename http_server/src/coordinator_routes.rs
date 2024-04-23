use std::sync::Arc;
use rocket::serde::json::Json;
use rocket::State;
use tokio::sync::RwLock;
use ws::{Message, WebSocket};
use algomanserver::Coordinator;
use crate::{Error, models};
use crate::models::{AgentKeyRequest, RegistrationResponse};
use crate::ws_helpers::ws_wait_for;
use rocket::futures::{SinkExt, StreamExt};

#[get("/lobbies")]
pub async fn lobbies(coordinator: &State<Arc<RwLock<Coordinator>>>) -> Json<Vec<models::Lobby>> {
    let coordinator = coordinator.read().await;
    let public_lobby_info: Vec<models::Lobby> = coordinator.lobbies().map(|l| models::Lobby {
        id: l.id,
        name: l.name.clone(),
        agents: l.agent_ids.iter().filter(|a| coordinator.get_agent(**a).is_some()).map(|a| {
            let agent = coordinator.get_agent(*a).expect("missing agents already should be filtered out");

            models::Agent {
                id: agent.id,
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

    Json(RegistrationResponse {
        agent_id,
        agent_key,
    })
}


#[post("/create_lobby", format = "json", data = "<data>")]
pub async fn create_lobby(coordinator: &State<Arc<RwLock<Coordinator>>>, data: Json<models::AgentKeyRequest>) -> Result<String, Error> {
    let mut coordinator = coordinator.write().await;

    match coordinator.create_lobby_with_host(data.agent_key, "Lobby").await {
        Ok(_) => {
            Ok("Agent created lobby, then joined it.".to_string())
        }
        Err(err) => {
            Err(err.into())
        }
    }
}


#[post("/join_lobby", format = "json", data = "<data>")]
pub async fn join_lobby(coordinator: &State<Arc<RwLock<Coordinator>>>, data: Json<models::AgentLobbyRequest>) -> Result<String, Error> {
    let mut coordinator = coordinator.write().await;

    match coordinator.join_lobby(data.agent_key, data.lobby_id).await {
        Ok(_) => {
            Ok(format!("Agent joined lobby {}", data.lobby_id))
        }
        Err(err) => {
            Err(err.into())
        }
    }
}


#[post("/leave_lobby", format = "json", data = "<data>")]
pub async fn leave_lobby(coordinator: &State<Arc<RwLock<Coordinator>>>, data: Json<models::AgentKeyRequest>) -> Result<String, Error> {
    let mut coordinator = coordinator.write().await;

    match coordinator.leave_current_lobby(data.agent_key).await {
        Ok(_) => {
            Ok("Agent left lobby".to_string())
        }
        Err(err) => {
            Err(err.into())
        }
    }
}

#[get("/lobby/<lobby_id>/listen")]
pub async fn lobby_listen(ws: WebSocket, coordinator: &State<Arc<RwLock<Coordinator>>>, lobby_id: u64) -> ws::Channel<'static> {

    let coordinator = coordinator.inner().clone();
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let (mut tx, mut rx) = stream.split();

            let agent_key = match ws_wait_for::<AgentKeyRequest>("agent key", &mut tx, &mut rx).await {
                None => return Ok(()),
                Some(model) => model.agent_key
            };

            let mut lobby_rx = {
                let mut coordinator = coordinator.write().await;
                match coordinator.lobby_listen(agent_key, lobby_id.into()) {
                    Ok(lobby_rx) => lobby_rx,
                    Err(err) => {
                        tx.send(Message::Text(format!("{}", err))).await.ok();
                        tx.send(Message::Close(None)).await.ok();
                        return Ok(());
                    }
                }
            };

            let mut send_task = tokio::spawn(async move {
                while let Some(lobby_event) = lobby_rx.recv().await {
                    let event_json = serde_json::to_string(&lobby_event).expect("serialized lobby event");
                    let _ = tx.send(Message::Text(event_json)).await;
                }
            });

            let mut recv_task = tokio::spawn(async move {
                while let Some(message) = rx.next().await {
                    if let Ok(message) = message {
                        match message {
                            Message::Text(_) => {}
                            Message::Binary(_) => {}
                            Message::Ping(_) => {}
                            Message::Pong(_) => {}
                            Message::Close(_) => {
                                return;
                            }
                            Message::Frame(_) => {}
                        }
                        println!("received message")
                    }
                }
            });

            // If any one of the tasks exit, abort the other.
            tokio::select! {
                _ = (&mut send_task) => {
                    recv_task.abort();
                },
                _ = (&mut recv_task) => {
                    send_task.abort();
                }
            }

            Ok(())
        })
    })
}
