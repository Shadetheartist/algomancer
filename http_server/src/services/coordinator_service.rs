use algomanserver::Coordinator;
use crate::error::Error;
use crate::models;
use crate::models::LobbyModel;

pub fn lobbies(coordinator: &Coordinator) -> Vec<LobbyModel> {
    coordinator.lobbies().map(|l| LobbyModel::from_coordinator_lobby(coordinator, l)).collect()
}

pub async fn register(coordinator: &mut Coordinator, username: &str) -> Result<models::RegistrationResponse, Error> {
    let (agent_id, agent_key) = coordinator.create_new_agent(username).await?;

    println!("registered agent {agent_key}");

    let model = models::RegistrationResponse {
        agent_id,
        agent_key: agent_key.to_string(),
    };

    Ok(model)
}