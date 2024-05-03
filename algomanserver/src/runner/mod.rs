pub mod client;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use algomancer_gre::game::{Game};
use algomancer_gre::game::game_builder::NewGameError;
use crate::{AgentId, AgentKey, Lobby, LobbyEvent};
use crate::runner::client::ClientKey;

#[derive(Debug)]
pub enum Error {
    NewGameError(NewGameError),
    CouldNotMigrate(AgentId, Box<crate::coordinator::Error>),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub struct RunnerId(pub u64);

impl Display for RunnerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl From<u64> for  RunnerId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}


#[derive(Debug)]
pub struct Runner {
    pub runner_id: RunnerId,
    migration_state: Option<MigrationState>,
    run_tx: Option<tokio::sync::broadcast::Sender<()>>,
    game: Game
}

#[derive(Debug)]
pub struct MigrationState {
    migration_keys: HashMap<AgentKey, ClientKey>,
    clients: HashMap<ClientKey, bool> // connected
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct MigrationInfo {
    pub runner_id: RunnerId,
    pub agent_key: AgentKey,
    pub client_key: ClientKey,
}

impl Runner {
    /// creates a game runner instance from a coordinator lobby
    /// this will send migration instructions to all agents in the lobby before returning.
    /// if there is an error in the process of creating the runner or sending instructions, this
    /// returns an error. The expectation is that after this, consumers will organize connecting
    /// clients who reach out based on the instructions given to the runner.
    /// once all agents connect, the runner can start. And the lobby can be cleaned up.
    pub async fn from_lobby(lobby: &Lobby, lobby_agent_keys: Vec<(AgentId, AgentKey)>) -> Result<Self, Error> {

        let game = match Game::new(&lobby.game_options) {
            Ok(game) => game,
            Err(err) => return Err(Error::NewGameError(err))
        };

        let mut runner = Self {
            runner_id: RunnerId(rand::thread_rng().next_u64()),
            migration_state: None,
            run_tx: Default::default(),
            game,
        };

        runner.migration_state = match runner.begin_migration(lobby, lobby_agent_keys).await {
            Ok(migration_state) => Some(migration_state),
            Err(err) => {
                return Err(err);
            }
        };

        Ok(runner)
    }

    pub fn run(&self) {
        println!("run game server!");
        self.run_tx.as_ref().unwrap().send(()).unwrap();
    }

    pub fn ready(&self) -> bool {
        if let Some(migration_state) = &self.migration_state {
            let connected_clients_count = migration_state.clients.iter().filter(|(_, connected)| **connected).count();
            connected_clients_count == migration_state.migration_keys.len()
        } else {
            false
        }
    }

    pub fn connect_client(&mut self, client_key: ClientKey) -> tokio::sync::broadcast::Receiver<()> {
        if let Some(migration_state) = &mut self.migration_state {
            if let Some((_, client_key)) = migration_state.migration_keys.iter().find(|(_, k)| **k == client_key) {
                migration_state.clients.insert(*client_key, true);

                let rx = if let Some(ready_tx) = &mut self.run_tx {
                    ready_tx.subscribe()
                } else {
                    let (tx, rx) = tokio::sync::broadcast::channel(1);
                    self.run_tx = Some(tx);
                    rx
                };

                if self.ready() {
                    self.run();
                }

                return rx;
            }

            panic!("invalid client");

        } else {
            panic!("no migration state")
        }
    }

    pub fn disconnect_client(&mut self, client_key: ClientKey) {
        if let Some(migration_state) = &mut self.migration_state {
            migration_state.clients.remove(&client_key).expect("a removed client");
        } else {
            panic!("no migration state")
        }
    }


    async fn begin_migration(&self, lobby: &Lobby, lobby_agent_keys: Vec<(AgentId, AgentKey)>) -> Result<MigrationState, Error> {

        let migration_keys = lobby_agent_keys.iter().fold(HashMap::new(), |mut map, a| {
            map.insert(a.1, ClientKey::random());
            map
        });

        for (agent_id, agent_key) in &lobby_agent_keys {
            let info = MigrationInfo {
                runner_id: self.runner_id,
                agent_key: *agent_key,
                client_key: *migration_keys.get(agent_key).expect("a migration key for this agent")
            };

            match lobby.send_event(LobbyEvent::Migrate(*agent_id, info)).await {
                Ok(_) => {}
                Err(err) => {
                    return Err(Error::CouldNotMigrate(*agent_id, Box::new(err)));
                }
            }
        }

        let migration_state = MigrationState {
            migration_keys,
            clients: Default::default()
        };

        Ok(migration_state)
    }

}