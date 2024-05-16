mod models;
mod coordinator_routes;
mod messages;
mod ws;
mod runner_routes;
mod error;
mod services;

#[macro_use]
extern crate rocket;

use std::sync::Arc;
use tokio::sync::RwLock;
use algomanserver::Coordinator;


#[launch]
#[tokio::main]
async fn rocket() -> _ {

    let coordinator = Coordinator::new();
    let coordinator_rwl = RwLock::new(coordinator);
    let coordinator_arc: Arc<RwLock<Coordinator>> = Arc::new(coordinator_rwl);

    generate_test_lobbies(&coordinator_arc);

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

fn generate_test_lobbies(coordinator_arc: &Arc<RwLock<Coordinator>>) {
    // simulate some state to test
    let coordinator_arc_clone = coordinator_arc.clone();
    tokio::spawn(async move {
        let mut coordinator = coordinator_arc_clone.write().await;

        let mut a_id = 0;
        for i in 0..100 {
            let (_agent_id, agent_key) = coordinator.create_new_agent(format!("Agent {a_id}").as_str()).await.unwrap();
            a_id += 1;

            let lobby_id = coordinator.create_lobby_with_host(agent_key, format!("Lobby {i}").as_str()).await.unwrap();

            for _a in 1..=(rand::random::<u64>() % 4) {
                let (_agent_id, agent_key) = coordinator.create_new_agent(format!("Agent {}", a_id).as_str()).await.unwrap();
                a_id += 1;

                coordinator.join_lobby(agent_key, lobby_id).await.unwrap()
            }
        }
    });
}
