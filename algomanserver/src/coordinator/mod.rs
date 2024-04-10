pub mod agent;
pub mod lobby;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::Receiver;
use algomancer_gre::game::{GameOptions};
use algomancer_gre::game::state::GameMode;
use algomancer_gre::game::state::rng::AlgomancerRngSeed;
use crate::coordinator::agent::{Agent, AgentId, AgentKey};
use crate::coordinator::Error::CannotRunError;
use crate::coordinator::lobby::{Lobby, LobbyEvent, LobbyEventType, LobbyId};
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

    pub fn create_new_agent(&mut self, username: &str) -> (AgentId, AgentKey) {
        let id = self.next_agent_id();

        let agent = Agent::new(id, username.to_string());
        let key = agent.key;

        self.agents.push(agent);

        self.last_agent_id = id;

        (id, key)
    }

    pub fn create_lobby_with_host(&mut self, host_agent_key: AgentKey) -> Result<LobbyId, Error> {

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
            runner: None,
            options,
            host_agent_id: agent.id,
            agents: vec![host_agent_id],
            broadcast: tx,
        };

        self.lobbies.push(lobby);

        self.last_lobby_id = lobby_id;

        Ok(lobby_id)
    }

    pub fn leave_current_lobby(&mut self, leaver_agent_key: AgentKey) -> Result<(), Error> {
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

            if current_lobby.agents.is_empty() {
                // if the lobby is empty, remove the lobby (after borrow is over)
                remove_lobby = Some(current_lobby.id);
            } else {
                // if the leaver was the host - assign a new host
                if current_lobby.host_agent_id == leaver_agent_id {
                    let next_host_agent_id = current_lobby.agents.first().expect("another player");
                    current_lobby.host_agent_id = *next_host_agent_id;
                }

                Self::broadcast_lobby_event(current_lobby, LobbyEvent {
                    event_type: LobbyEventType::AgentLeft,
                    event_arg: leaver_agent_id.to_string(),
                });
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

    pub fn join_lobby(&mut self, agent_key: AgentKey, lobby_id: LobbyId) -> Result<(), Error> {
        let agent_id = match self.must_get_agent_id_by_key(agent_key) {
            Ok(agent_id) => agent_id,
            Err(e) => return Err(e)
        };

        match self.leave_current_lobby(agent_key) {
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

        Self::broadcast_lobby_event(lobby, LobbyEvent {
            event_type: LobbyEventType::AgentJoined,
            event_arg: agent_id.to_string(),
        });

        Ok(())
    }

    fn broadcast_lobby_event(lobby: &Lobby, lobby_event: LobbyEvent){
        match lobby.broadcast.send(lobby_event.clone()) {
            Ok(_) => {
                println!("broadcast {:?}", lobby_event);
            }
            Err(_) => {
                // can only fail when there are no active receivers, which is actually totally fine
                //eprintln!("err {} when broadcasting {:?}", err, lobby_event);
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

    pub fn lobby_listen(&self, lobby_id: LobbyId) -> Result<Receiver<LobbyEvent>, Error> {
        let lobby = match self.get_lobby(lobby_id) {
            Some(lobby) => lobby,
            None => return Err(Error::LobbyDoesNotExist(lobby_id)),
        };

        let receiver = lobby.broadcast.subscribe();

        Ok(receiver)
    }

    pub fn start_game(&mut self, lobby_id: LobbyId) -> Result<Arc<Mutex<Runner>>, Error> {
        let lobby = match self.get_lobby_mut(lobby_id) {
            Some(lobby) => lobby,
            None => return Err(Error::LobbyDoesNotExist(lobby_id)),
        };

        let runner = match Runner::new(lobby_id, &lobby.options) {
            Ok(runner) => runner,
            Err(err) => return Err(CannotRunError(err))
        };

        let runner_mutex = Mutex::new(runner);
        let arc = Arc::new(runner_mutex);

        lobby.runner = Some(arc.clone());

        Ok(arc)
    }
}

#[cfg(test)]
mod tests {
    use crate::coordinator::{Coordinator};

    #[test]
    fn test_coordinator_join_leave() {
        let mut coordinator = Coordinator::new();

        let (_, agent_key) = coordinator.create_new_agent("Jim");
        println!("agent 1 {:?}", agent_key);

        let lobby_id = coordinator.create_lobby_with_host(agent_key).unwrap();
        println!("lobby_id {:?}", lobby_id);

        let (_, agent_2_key) = coordinator.create_new_agent("Pam");
        println!("agent 2 {:?}", agent_2_key);

        coordinator.join_lobby(agent_2_key, lobby_id).unwrap();

        coordinator.leave_current_lobby(agent_key).unwrap();

        for l in coordinator.lobbies() {
            println!("{:?}", l);
        }

        let mut agent_keys = vec![];
        agent_keys.push(agent_2_key);

        let (_, agent_key) = coordinator.create_new_agent("Dwight");
        coordinator.join_lobby(agent_key, lobby_id).unwrap();
        agent_keys.push(agent_key);

        let (_, agent_key) = coordinator.create_new_agent("Larry");
        coordinator.join_lobby(agent_key, lobby_id).unwrap();
        agent_keys.push(agent_key);

        let (_, agent_key) = coordinator.create_new_agent("Denis");
        coordinator.join_lobby(agent_key, lobby_id).unwrap();
        agent_keys.push(agent_key);

        for l in coordinator.lobbies() {
            println!("{:?}", l);
        }

        for k in agent_keys {
            coordinator.leave_current_lobby(k).unwrap();
        }

        // lobby will self-delete when last player leaves
        assert!(coordinator.join_lobby(agent_key, lobby_id).is_err());
    }

    #[test]
    fn test_coordinator_2p_game() {
        let mut coordinator = Coordinator::new();

        let (_, agent_key) = coordinator.create_new_agent("Denis");
        println!("agent 1 {:?}", agent_key);

        let lobby_id = coordinator.create_lobby_with_host(agent_key).unwrap();
        println!("lobby_id {:?}", lobby_id);

        let (_, agent_2_key) = coordinator.create_new_agent("Greg");
        println!("agent 2 {:?}", agent_2_key);

        coordinator.join_lobby(agent_2_key, lobby_id).unwrap();

        let _runner_arc_mutex = coordinator.start_game(lobby_id).unwrap();

        print!("");
    }
}