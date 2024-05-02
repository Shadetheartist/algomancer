mod models;
mod coordinator_routes;
mod messages;
mod ws;
mod runner_routes;

#[macro_use]
extern crate rocket;

use std::sync::Arc;
use tokio::sync::RwLock;
use algomanserver::Coordinator;
use rand::RngCore;

use rocket::response::status;

use rocket::futures::{SinkExt, StreamExt};

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

    #[response(status = 400)]
    AgentAlreadyInLobby(String),

    #[response(status = 400)]
    AgentNotInCorrectLobby(String),

    #[response(status = 400)]
    AgentNotListeningToLobby(String),

    #[response(status = 400)]
    DuplicateUsername(String),

    #[response(status = 500)]
    SendEventError(String),

    #[response(status = 500)]
    CannotRunServer(String),

    #[response(status = 400)]
    LobbyIsFull(String),
}


impl From<algomanserver::coordinator::Error> for Error {
    fn from(value: algomanserver::coordinator::Error) -> Self {
        match value {
            algomanserver::coordinator::Error::AgentDoesNotExist(_) => {
                Error::AgentDoesNotExist(value.to_string())
            }
            algomanserver::coordinator::Error::AgentDoesNotExistWithKey(_) => {
                Error::AgentDoesNotExistWithKey(value.to_string())
            }
            algomanserver::coordinator::Error::LobbyDoesNotExist(_) => {
                Error::LobbyNotFound(value.to_string())
            }
            algomanserver::coordinator::Error::AgentNotInAnyLobby(_) => {
                Error::AgentNotInLobby(value.to_string())
            }
            algomanserver::coordinator::Error::AgentAlreadyInLobby(_, _) => {
                Error::AgentAlreadyInLobby(value.to_string())
            }
            algomanserver::coordinator::Error::CannotRunError(_) => {
                Error::CannotRunServer(value.to_string())
            }
            algomanserver::coordinator::Error::AgentNotInCorrectLobby(_) => {
                Error::AgentNotInCorrectLobby(value.to_string())
            }
            algomanserver::coordinator::Error::NotListening(_) => {
                Error::AgentNotInCorrectLobby(value.to_string())
            }
            algomanserver::coordinator::Error::SendEventError(_) => {
                Error::AgentNotInCorrectLobby(value.to_string())
            }
            algomanserver::coordinator::Error::AgentAlreadyExistsWithUsername => {
                Error::DuplicateUsername(value.to_string())
            }
            algomanserver::coordinator::Error::LobbyIsFull(_) => {
                Error::LobbyIsFull(value.to_string())
            }
        }
    }
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {


    let mut coordinator = Coordinator::new();
    let coordinator_rwl = RwLock::new(coordinator);
    let coordinator_arc: Arc<RwLock<Coordinator>> = Arc::new(coordinator_rwl);

    let coordinator_arc_clone = coordinator_arc.clone();
    tokio::spawn(async move {
        let mut coordinator = coordinator_arc_clone.write().await;

        // simulate some state to test
        let mut a_id = 0;
        for i in 0..100 {
            let (agent_id, agent_key) = coordinator.create_new_agent(format!("Agent {a_id}").as_str()).await.unwrap();
            a_id += 1;

            let lobby_id = coordinator.create_lobby_with_host(agent_key, format!("Lobby {i}").as_str()).await.unwrap();

            for a in 1..=(rand::random::<u64>() % 4) {
                let (agent_id, agent_key) = coordinator.create_new_agent(format!("Agent {}", a_id).as_str()).await.unwrap();
                a_id += 1;

                coordinator.join_lobby(agent_key, lobby_id).await.unwrap()
            }
        }
    });

    let runners : Vec<algomanserver::Runner> = Vec::new();
    let runners_rwl = RwLock::new(runners);
    let runners_arc: Arc<RwLock<Vec<algomanserver::Runner>>> = Arc::new(runners_rwl);

    rocket::build()
        .manage(coordinator_arc)
        .manage(runners_arc)
        .mount("/coordinator/", routes![
            coordinator_routes::register,
            coordinator_routes::lobbies,
            coordinator_routes::lobby_create,
            coordinator_routes::lobby_join,
        ])
        .mount("/runner/", routes![
            runner_routes::runner_connect,
        ])
}
