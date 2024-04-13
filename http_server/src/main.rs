mod models;

#[macro_use]
extern crate rocket;

use std::borrow::Cow;
use std::sync::Arc;
use rocket::{Response, State};
use tokio::sync::RwLock;
use algomanserver::{Agent, AgentId, Coordinator, Lobby, LobbyEvent, LobbyId};
use rand::{random, RngCore};
use rocket::async_stream::stream;
use rocket::http::{ContentType, Status};
use rocket::response::status;
use rocket::serde::json::Json;
use tokio::sync::broadcast::Receiver;
use ws::{Message, WebSocket};
use ws::frame::{CloseCode, CloseFrame};
use crate::models::RegistrationResponse;


#[derive(Debug, Responder)]
enum Error {
    #[response(status = 400)]
    BadRequest(String),

    #[response(status = 404)]
    LobbyNotFound(String),

    #[response(status = 500)]
    AgentDoesNotExist(String),

    #[response(status = 401)]
    AgentDoesNotExistWithKey(String),

    #[response(status = 400)]
    AgentNotInLobby(String),

    #[response(status = 500)]
    CannotRunServer(String),

}


impl From<algomanserver::Error> for Error {
    fn from(value: algomanserver::Error) -> Self {
        match value {
            algomanserver::Error::AgentDoesNotExist(_) => {
                Error::AgentDoesNotExist(value.to_string())
            }
            algomanserver::Error::AgentDoesNotExistWithKey(_) => {
                Error::AgentDoesNotExistWithKey(value.to_string())
            }
            algomanserver::Error::LobbyDoesNotExist(_) => {
                Error::LobbyNotFound(value.to_string())
            }
            algomanserver::Error::AgentNotInLobby(_) => {
                Error::AgentNotInLobby(value.to_string())
            }
            algomanserver::Error::CannotRunError(_) => {
                Error::CannotRunServer(value.to_string())
            }
        }
    }
}


#[get("/lobbies")]
async fn lobbies(coordinator: &State<Arc<RwLock<Coordinator>>>) -> Json<Vec<models::Lobby>> {
    let coordinator = coordinator.read().await;
    let public_lobby_info: Vec<models::Lobby> = coordinator.lobbies().map(|l| models::Lobby {
        id: l.id,
        name: "".to_string(),
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

#[post("/register", format="json", data = "<data>")]
async fn register(coordinator: &State<Arc<RwLock<Coordinator>>>, data: Json<models::RegistrationRequest>) -> Json<models::RegistrationResponse> {
    let mut coordinator = coordinator.write().await;

    let (agent_id, agent_key) = coordinator.create_new_agent(data.username.as_str());

    Json(RegistrationResponse {
        agent_id,
        agent_key,
    })
}


#[post("/create_lobby", format="json", data = "<data>")]
async fn create_lobby(coordinator: &State<Arc<RwLock<Coordinator>>>, data: Json<models::AgentKeyRequest>) -> Result<String, Error> {
    let mut coordinator = coordinator.write().await;

    match coordinator.create_lobby_with_host(data.agent_key) {
        Ok(_) => {
            Ok("Agent created lobby, then joined it.".to_string())
        }
        Err(err) => {
            Err(err.into())
        }
    }
}


#[post("/join_lobby", format="json", data = "<data>")]
async fn join_lobby(coordinator: &State<Arc<RwLock<Coordinator>>>, data: Json<models::AgentLobbyRequest>) -> Result<String, Error> {
    let mut coordinator = coordinator.write().await;

    match coordinator.join_lobby(data.agent_key, data.lobby_id) {
        Ok(_) => {
            Ok(format!("Agent joined lobby {}", data.lobby_id))
        }
        Err(err) => {
            Err(err.into())
        }
    }
}


#[post("/leave_lobby", format="json", data = "<data>")]
async fn leave_lobby(coordinator: &State<Arc<RwLock<Coordinator>>>, data: Json<models::AgentKeyRequest>) -> Result<String, Error> {
    let mut coordinator = coordinator.write().await;

    match coordinator.leave_current_lobby(data.agent_key) {
        Ok(_) => {
            Ok("Agent left lobby".to_string())
        }
        Err(err) => {
            Err(err.into())
        }
    }
}


#[get("/lobby/<lobby_id>/listen")]
async fn lobby_listen(ws: WebSocket, coordinator: &State<Arc<RwLock<Coordinator>>>, lobby_id: u64) -> ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};

    let mut coordinator = coordinator.write().await;

    let mut lobby_events_rx = match coordinator.lobby_listen(lobby_id.into()) {
        Ok(rx) => rx,
        Err(_) => {
            return ws.channel(move |mut stream| Box::pin(async move {
                Ok(())
            }));
        }
    };

    ws.channel(move |mut stream| Box::pin(async move {

        let (mut tx, mut rx) = stream.split();

        let mut send_task = tokio::spawn(async move {
            loop {
                if let Ok(lobby_event) = lobby_events_rx.recv().await {
                    let event_json = serde_json::to_string(&lobby_event).expect("serialized lobby event");
                    let _ = tx.send(Message::Text(event_json)).await;
                }
            }
        });

        let mut recv_task = tokio::spawn(async move {
            while let Some(message) = rx.next().await {
                if let Ok(message) = message {
                    println!("received message")
                }
            }
        });

        // If any one of the tasks exit, abort the other.
        tokio::select! {
            rv_a = (&mut send_task) => {
                recv_task.abort();
            },
            rv_b = (&mut recv_task) => {
                send_task.abort();
            }
        }

        Ok(())
    }))
}


#[launch]
fn rocket() -> _ {
    let mut coordinator = Coordinator::new();

    // simulate some state to test
    for i in 0..100 {
        let (agent_id, agent_key) = coordinator.create_new_agent(format!("Agent {i}").as_str());
        let lobby_id = coordinator.create_lobby_with_host(agent_key).unwrap();

        for a in 1..(rand::thread_rng().next_u32() % 4) {
            let (agent_id, agent_key) = coordinator.create_new_agent(format!("Agent {}", a + i).as_str());
            coordinator.join_lobby(agent_key, lobby_id).unwrap()
        }
    }

    let coordinator_rwl = RwLock::new(coordinator);
    let coordinator_arc: Arc<RwLock<Coordinator>> = Arc::new(coordinator_rwl);

    rocket::build()
        .manage(coordinator_arc)
        .mount("/coordinator/", routes![
            register,
            create_lobby,
            join_lobby,
            leave_lobby,
            lobby_listen,
            lobbies,
        ])
}
