mod controller;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use algomancer_gre::game::{Game};
use algomancer_gre::game::game_builder::NewGameError;
use algomancer_gre::game::state::player::PlayerId;
use crate::{AgentId, AgentKey, Lobby, LobbyEvent};
use crate::runner::controller::ControllerKey;

#[derive(Debug)]
pub enum Error {
    NewGameError(NewGameError)
}

#[derive(Debug)]
pub struct Runner {
    game: Game
}

pub struct MigrationState {
    migration_keys: HashMap<AgentKey, ControllerKey>,
    controllers: HashMap<ControllerKey, PlayerId>
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct MigrationInfo {

}

impl Runner {
    pub fn new(lobby: &Lobby, lobby_agent_keys: Vec<(AgentId, AgentKey)>) -> Result<Self, Error> {
        // send out connection info to each client
        // wait for clients to connect
        // once all clients are connected, begin the game
        // loop
        //  wait for a valid action from a client
        //  apply action and broadcast change to clients
        //  if game ends then clean up
        // if clients disconnect unexpectedly - wait to reconnect process
        // if a clients disconnect on purpose, end remove the player and potentially end the game
        // game ends - escape runner and clean up

        let game = match Game::new(&lobby.options) {
            Ok(game) => game,
            Err(err) => return Err(Error::NewGameError(err))
        };

        let runner = Self {
            game,
        };

        Ok(runner)
    }

    pub fn run() {
        tokio::spawn(async move {

        });
    }

    fn wait_for_clients() {}

    async fn begin_migration(lobby: &Lobby, lobby_agent_keys: Vec<(AgentId, AgentKey)>) -> MigrationState {

        let migration_keys = lobby_agent_keys.iter().fold(HashMap::new(), |mut map, a| {
            map.insert(a.1, ControllerKey::random());
            map
        });

        let mut migration_state = MigrationState {
            migration_keys: migration_keys,
            controllers: Default::default()
        };

        for agent_id in &lobby.agent_ids {
            let info = MigrationInfo {

            };

            lobby.send_event(LobbyEvent::Migrate(agent_id.clone(), info)).await.unwrap();
        }

        migration_state
    }

}