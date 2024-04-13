pub mod agent;
pub mod lobby;

use std::fmt::{Display, Formatter};
use algomancer_gre::game::{GameOptions};
use algomancer_gre::game::state::GameMode;
use algomancer_gre::game::state::rng::AlgomancerRngSeed;
use crate::coordinator::agent::{Agent, AgentId, AgentKey};
use crate::coordinator::Error::CannotRunError;
use crate::coordinator::lobby::{Lobby, LobbyEvent, LobbyId};
use crate::runner::Runner;

#[derive(Debug)]
pub struct Coordinator {
    last_agent_id: AgentId,
    last_lobby_id: LobbyId,
    agents: Vec<Agent>,
    lobbies: Vec<Lobby>,
}

#[derive(Debug)]
pub enum Error {
    AgentDoesNotExist(AgentId),
    AgentDoesNotExistWithKey(AgentKey),
    LobbyDoesNotExist(LobbyId),
    AgentNotInLobby(AgentId),
    CannotRunError(crate::runner::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AgentDoesNotExist(agent_id) => {
                write!(f, "agent {agent_id} does not exist")
            }
            Error::AgentDoesNotExistWithKey(agent_key) => {
                write!(f, "agent does not exist with key {agent_key}")
            }
            Error::LobbyDoesNotExist(lobby_id) => {
                write!(f, "lobby {lobby_id} does not exist")
            }
            Error::AgentNotInLobby(agent_id) => {
                write!(f, "agent {agent_id} is not in a lobby")
            }
            CannotRunError(_) => {
                write!(f, "cannot run the game")
            }
        }
    }
}

impl std::error::Error for Error {}

impl Coordinator {
    pub fn new() -> Self {
        Self {
            last_agent_id: AgentId(0),
            last_lobby_id: LobbyId(0),
            agents: vec![],
            lobbies: vec![],
        }
    }

    pub async fn create_new_agent(&mut self, username: &str) -> (AgentId, AgentKey) {
        let id = self.next_agent_id();

        let agent = Agent::new(id, username.to_string());
        let key = agent.key;

        self.agents.push(agent);

        self.last_agent_id = id;

        (id, key)
    }

    pub async fn create_lobby_with_host(&mut self, host_agent_key: AgentKey) -> Result<LobbyId, Error> {

        let _ = self.leave_current_lobby(host_agent_key);

        let host_agent_id = match self.must_get_agent_id_by_key(host_agent_key) {
            Ok(agent_id) => agent_id,
            Err(e) => return Err(e)
        };

        let agent = match self.must_get_agent(host_agent_id) {
            Ok(agent) => agent,
            Err(e) => return Err(e)
        };

        let options = GameOptions {
            seed: AlgomancerRngSeed::from([0; 16]),
            game_mode: GameMode::new_player_mode(),
        };

        let lobby_id = self.next_game_id();

        let (tx, _) = tokio::sync::broadcast::channel::<LobbyEvent>(4);

        let lobby = Lobby {
            id: lobby_id,
            options,
            host_agent_id: agent.id,
            agents: vec![host_agent_id],
            target: Default::default(),
        };

        self.lobbies.push(lobby);

        self.last_lobby_id = lobby_id;

        Ok(lobby_id)
    }

    pub async fn leave_current_lobby(&mut self, leaver_agent_key: AgentKey) -> Result<(), Error> {
        let leaver_agent_id = match self.must_get_agent_id_by_key(leaver_agent_key) {
            Ok(agent_id) => agent_id,
            Err(e) => return Err(e)
        };

        if self.get_agent(leaver_agent_id).is_none() {
            return Err(Error::AgentDoesNotExist(leaver_agent_id));
        }

        // remove lobby if it's empty after the agent leaves, must be done after the mutable borrow is over
        let mut remove_lobby: Option<LobbyId> = None;

        if let Some(current_lobby) = self.get_current_lobby_mut(leaver_agent_id) {
            let agent_idx = current_lobby.agents
                .iter()
                .enumerate()
                .find(|(_, agent_id)| **agent_id == leaver_agent_id)
                .unwrap_or_else(|| panic!("a controller with an agent with id {:?}", leaver_agent_id)).0;

            current_lobby.agents.remove(agent_idx);
            current_lobby.target.remove(&leaver_agent_id);

            if current_lobby.agents.is_empty() {
                // don't need to send any events here as the lobby only had the one player and the lobby is about to close

                // if the lobby is empty, remove the lobby (after borrow is over)
                remove_lobby = Some(current_lobby.id);
            } else {
                Self::send_lobby_event(current_lobby, LobbyEvent::AgentLeft(leaver_agent_id)).await;

                // if the leaver was the host - assign a new host
                if current_lobby.host_agent_id == leaver_agent_id {
                    let next_host_agent_id = current_lobby.agents.first().expect("another player");
                    current_lobby.host_agent_id = *next_host_agent_id;
                    Self::send_lobby_event(current_lobby, LobbyEvent::NewHost(current_lobby.host_agent_id)).await;
                }
            }
        } else {
            return Err(Error::AgentNotInLobby(leaver_agent_id));
        }

        if let Some(lobby_id) = remove_lobby {
            let lobby_idx = self.lobbies.iter().enumerate().find(|(_, l)| l.id == lobby_id).expect("this lobby").0;
            self.lobbies.remove(lobby_idx);
        }

        Ok(())
    }

    pub async fn join_lobby(&mut self, agent_key: AgentKey, lobby_id: LobbyId) -> Result<(), Error> {
        let agent_id = match self.must_get_agent_id_by_key(agent_key) {
            Ok(agent_id) => agent_id,
            Err(e) => return Err(e)
        };

        match self.leave_current_lobby(agent_key).await {
            Ok(_) => {}
            Err(err) => {
                if let Error::AgentNotInLobby(_) = err {} else {
                    return Err(err);
                }
            }
        }

        let lobby = match self.get_lobby_mut(lobby_id) {
            Some(lobby) => lobby,
            None => return Err(Error::LobbyDoesNotExist(lobby_id)),
        };

        lobby.agents.push(agent_id);

        Self::send_lobby_event(lobby, LobbyEvent::AgentJoined(agent_id)).await;

        Ok(())
    }

    async fn send_lobby_event(lobby: &Lobby, lobby_event: LobbyEvent){

        match lobby_event {
            LobbyEvent::AgentJoined(_) |
            LobbyEvent::AgentLeft(_) |
            LobbyEvent::NewHost(_) => {
                println!("queuing broadcast of event to all lobby target channels: {:?}", lobby_event);
                for (_, rx) in &lobby.target {
                    rx.send(lobby_event.clone()).await.unwrap();
                }
            }

            LobbyEvent::Migrate(agent_id) |
            LobbyEvent::Whisper(_, agent_id, _) => {
                let target_tx = lobby.target.get(&agent_id).expect("agent to have a tx");

                match target_tx.send(lobby_event.clone()).await {
                    Ok(_) => {
                        println!("queued targeted message to channel: {:?}", lobby_event);
                    }
                    Err(err) => {
                        // can only fail when there are no active receivers, which is actually totally fine
                        eprintln!("err {} when queueing targeted message to channel: {:?}", err, lobby_event);
                    }
                }
            }
        }

    }

    pub fn get_agent(&self, agent_id: AgentId) -> Option<&Agent> {
        self.agents.iter().find(|a| a.id == agent_id)
    }

    pub fn get_lobby(&self, lobby_id: LobbyId) -> Option<&Lobby> {
        self.lobbies.iter().find(|l| l.id == lobby_id)
    }

    pub fn get_lobby_mut(&mut self, lobby_id: LobbyId) -> Option<&mut Lobby> {
        self.lobbies.iter_mut().find(|l| l.id == lobby_id)
    }

    pub fn get_current_lobby_mut(&mut self, agent_id: AgentId) -> Option<&mut Lobby> {
        self.lobbies
            .iter_mut()
            .find(|l| l.agents.iter().any(|a| *a == agent_id))
    }

    pub fn must_get_agent(&self, agent_id: AgentId) -> Result<&Agent, Error> {
        if let Some(agent) = self.agents.iter().find(|a| a.id == agent_id) {
            Ok(agent)
        } else {
            Err(Error::AgentDoesNotExist(agent_id))
        }
    }

    pub fn must_get_agent_id_by_key(&self, agent_key: AgentKey) -> Result<AgentId, Error> {
        if let Some(agent) = self.agents.iter().find(|a| a.key == agent_key) {
            Ok(agent.id)
        } else {
            Err(Error::AgentDoesNotExistWithKey(agent_key))
        }
    }

    fn next_game_id(&self) -> LobbyId {
        LobbyId(self.last_lobby_id.0 + 1)
    }

    fn next_agent_id(&self) -> AgentId {
        AgentId(self.last_agent_id.0 + 1)
    }

    pub fn lobbies(&self) -> std::slice::Iter<'_, Lobby> {
        self.lobbies.iter()
    }

    pub fn lobby_listen(&mut self, agent_key: AgentKey, lobby_id: LobbyId) -> Result<tokio::sync::mpsc::Receiver<LobbyEvent>, Error> {

        let agent_id = match self.must_get_agent_id_by_key(agent_key) {
            Ok(agent_id) => agent_id,
            Err(e) => return Err(e)
        };

        let lobby = match self.get_lobby_mut(lobby_id) {
            Some(lobby) => lobby,
            None => return Err(Error::LobbyDoesNotExist(lobby_id)),
        };

        let (a_tx, a_rx) = tokio::sync::mpsc::channel::<LobbyEvent>(4);

        lobby.target.insert(agent_id, a_tx);

        Ok(a_rx)
    }

    pub async fn whisper(&self, agent_key: AgentKey, target_agent_id: AgentId, content: String) -> Result<(), Error> {
        let agent_id = match self.must_get_agent_id_by_key(agent_key) {
            Ok(agent_id) => agent_id,
            Err(e) => return Err(e)
        };

        let lobby_id = match self.lobbies.iter().find(|l| l.agents.contains(&agent_id)) {
            None => {
                return Err(Error::AgentNotInLobby(agent_id))
            }
            Some(l) => l.id
        };

        let lobby = self.get_lobby(lobby_id).expect("a lobby");

        let event = LobbyEvent::Whisper(agent_id, target_agent_id, content);

        Self::send_lobby_event(lobby, event).await;

        Ok(())
    }

    pub async fn start_game(&mut self, lobby_id: LobbyId) -> Result<(), Error> {
        let lobby = match self.get_lobby_mut(lobby_id) {
            Some(lobby) => lobby,
            None => return Err(Error::LobbyDoesNotExist(lobby_id)),
        };

        let runner = match Runner::new(&lobby) {
            Ok(runner) => runner,
            Err(err) => return Err(CannotRunError(err))
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::sync::mpsc::Receiver;
    use tokio::task::{JoinHandle};
    use tokio::time::sleep;
    use crate::coordinator::{Coordinator};
    use crate::LobbyEvent;

    fn expect_events(mut rx: Receiver<LobbyEvent>, expected_events: Vec<LobbyEvent>) -> JoinHandle<()>{
        tokio::spawn(async move {
            for expected_event in expected_events {
                match rx.recv().await {
                    None => {}
                    Some(event) => {
                        if event == expected_event {
                            println!("received expected event {:?}", event);
                        } else {
                            panic!("received unexpected event\n{:?}\nExpected\n{:?}", event, expected_event)
                        }
                    }
                }

            }
        })
    }

    #[tokio::test]
    async fn test_coordinator_listen() {
        let mut coordinator = Coordinator::new();

        // users register
        let (jim_agent_id, jim_agent_key) = coordinator.create_new_agent("Jim").await;
        let (pam_agent_id, pam_agent_key) = coordinator.create_new_agent("Pam").await;
        let (dwight_agent_id, dwight_agent_key) = coordinator.create_new_agent("Dwight").await;


        // jim makes lobby
        let lobby_id = coordinator.create_lobby_with_host(jim_agent_key).await.unwrap();

        // jim listens to lobby
        let jim_rx = coordinator.lobby_listen(jim_agent_key, lobby_id).unwrap();

        // pam joins jim's lobby
        coordinator.join_lobby(pam_agent_key, lobby_id).await.unwrap();

        // pam listens to lobby
        let pam_rx = coordinator.lobby_listen(pam_agent_key, lobby_id).unwrap();

        coordinator.whisper(jim_agent_key, pam_agent_id, "Hello Pam".to_string()).await.unwrap();

        let dwight_rx = coordinator.lobby_listen(pam_agent_key, lobby_id).unwrap();
        coordinator.join_lobby(dwight_agent_key, lobby_id).await.unwrap();

        coordinator.whisper(pam_agent_key, jim_agent_id, "Hello Jim".to_string()).await.unwrap();

        let jim_listener_handle = expect_events(jim_rx, vec![
            LobbyEvent::AgentJoined(pam_agent_id),
            LobbyEvent::AgentJoined(dwight_agent_id),
            LobbyEvent::Whisper(pam_agent_id, jim_agent_id, "Hello Jim".to_string()),
        ]);

        let pam_listener_handle = expect_events(pam_rx, vec![
            LobbyEvent::Whisper(jim_agent_id, pam_agent_id, "Hello Pam".to_string()),
            LobbyEvent::AgentJoined(dwight_agent_id),
        ]);

        let timeout = async {
            sleep(Duration::from_millis(100)).await
        };

        tokio::select! {
            _ = timeout => {
                panic!("ran out of time to listen for the events")
            },
            err = jim_listener_handle => {
                match err {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("jim's event list was wrong: {:?}", err)
                    }
                }
            },
            err = pam_listener_handle => {
                match err {
                    Ok(e) => {}
                    Err(err) => {
                        panic!("pam's event list was wrong: {:?}", err)
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_coordinator_broadcast() {
        let mut coordinator = Coordinator::new();
        let (jim_agent_id, jim_agent_key) = coordinator.create_new_agent("Jim").await;
        let lobby_id = coordinator.create_lobby_with_host(jim_agent_key).await.unwrap();

        let rx = coordinator.lobby_listen(jim_agent_key, lobby_id).unwrap();

        let (pam_agent_id, pam_agent_key) = coordinator.create_new_agent("Pam").await;

        let listener_handle = expect_events(rx, vec![
            LobbyEvent::AgentJoined(pam_agent_id),
            LobbyEvent::AgentLeft(jim_agent_id),
            LobbyEvent::NewHost(pam_agent_id),
            LobbyEvent::AgentJoined(jim_agent_id),
        ]);

        coordinator.join_lobby(pam_agent_key, lobby_id).await.unwrap();
        coordinator.leave_current_lobby(jim_agent_key).await.unwrap();
        coordinator.join_lobby(jim_agent_key, lobby_id).await.unwrap();

        if let Err(err) = listener_handle.await {
            panic!("{:?}", err);
        }

    }

    #[tokio::test]
    async fn test_coordinator_join_leave() {
        let mut coordinator = Coordinator::new();

        let (_, agent_key) = coordinator.create_new_agent("Jim").await;

        let lobby_id = coordinator.create_lobby_with_host(agent_key).await.unwrap();

        let (_, agent_2_key) = coordinator.create_new_agent("Pam").await;

        coordinator.join_lobby(agent_2_key, lobby_id).await.unwrap();

        coordinator.leave_current_lobby(agent_key).await.unwrap();

        let mut agent_keys = vec![];
        agent_keys.push(agent_2_key);

        let (_, agent_key) = coordinator.create_new_agent("Dwight").await;
        coordinator.join_lobby(agent_key, lobby_id).await.unwrap();
        agent_keys.push(agent_key);

        let (_, agent_key) = coordinator.create_new_agent("Larry").await;
        coordinator.join_lobby(agent_key, lobby_id).await.unwrap();
        agent_keys.push(agent_key);

        let (_, agent_key) = coordinator.create_new_agent("Denis").await;
        coordinator.join_lobby(agent_key, lobby_id).await.unwrap();
        agent_keys.push(agent_key);

        for k in agent_keys {
            coordinator.leave_current_lobby(k).await.unwrap();
        }

        // lobby will self-delete when last player leaves
        assert!(coordinator.join_lobby(agent_key, lobby_id).await.is_err());
    }

    #[tokio::test]
     async fn test_coordinator_2p_game() {
        let mut coordinator = Coordinator::new();

        let (_, agent_key) = coordinator.create_new_agent("Denis").await;

        let lobby_id = coordinator.create_lobby_with_host(agent_key).await.unwrap();

        let (_, agent_2_key) = coordinator.create_new_agent("Greg").await;

        coordinator.join_lobby(agent_2_key, lobby_id).await.unwrap();

        coordinator.start_game(lobby_id).await.unwrap();
    }
}