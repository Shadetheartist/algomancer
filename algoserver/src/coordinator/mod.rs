pub mod service;

use std::fmt::{Display, Formatter, write, Write};
use std::sync::{Arc, Mutex};
use algomancer_gre::game::{GameOptions};
use algomancer_gre::game::state::GameMode;
use algomancer_gre::game::state::rng::AlgomancerRngSeed;
use crate::coordinator::Error::CannotRunError;
use crate::runner::Runner;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AgentId(u64);

impl Display for AgentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

impl From<u64> for AgentId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct LobbyId(u64);

impl Display for LobbyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}


impl From<u64> for LobbyId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}


#[derive(Debug)]
pub enum Error {
    AgentDoesNotExist(AgentId),
    LobbyDoesNotExist(LobbyId),
    AgentNotInLobby(AgentId),
    CannotRunError(crate::runner::Error)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AgentDoesNotExist(agent_id) => {
                write!(f, "agent {agent_id} does not exist")
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

impl std::error::Error for Error {

}

#[derive(Debug)]
pub struct Lobby {
    id: LobbyId,

    runner: Option<Arc<Mutex<Runner>>>,

    options: GameOptions,

    host_agent_id: AgentId,
    agents: Vec<AgentId>,
}

#[derive(Debug)]
pub struct Coordinator {
    last_agent_id: AgentId,
    last_lobby_id: LobbyId,
    agents: Vec<Agent>,
    lobbies: Vec<Lobby>,
}

#[derive(Debug)]
pub struct Agent {
    id: AgentId,
    username: String
}

impl Coordinator {
    pub fn new() -> Self {
        Self {
            last_agent_id: AgentId(0),
            last_lobby_id: LobbyId(0),
            agents: vec![],
            lobbies: vec![],
        }
    }

    pub fn create_new_agent(&mut self, username: &str) -> AgentId {
        let id = self.next_agent_id();
        let agent = Agent {
            id: id,
            username: username.to_string()
        };

        self.agents.push(agent);

        self.last_agent_id = id;

        id
    }

    pub fn create_lobby_with_host(&mut self, host_agent_id: AgentId) -> Result<LobbyId, Error> {

        let _ = self.leave_current_lobby(host_agent_id);

        let agent = match self.must_get_agent(host_agent_id) {
            Ok(agent) => agent,
            Err(e) => return Err(e)
        };


        let options = GameOptions {
            seed: AlgomancerRngSeed::from([0; 16]),
            game_mode: GameMode::new_player_mode(),
        };

        let lobby_id = self.next_game_id();

        let ongoing_game = Lobby {
            id: lobby_id,
            runner: None,
            options: options,
            host_agent_id: agent.id,
            agents: vec![host_agent_id],
        };

        self.lobbies.push(ongoing_game);

        self.last_lobby_id = lobby_id;

        Ok(lobby_id)
    }

    pub fn leave_current_lobby(&mut self, leaver_agent_id: AgentId) -> Result<(), Error> {
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
                .expect(format!("a controller with an agent with id {:?}", leaver_agent_id).as_str()).0;

            current_lobby.agents.remove(agent_idx);

            if current_lobby.agents.len() == 0 {
                // if the lobby is empty, remove the lobby (after borrow is over)
                remove_lobby = Some(current_lobby.id);
            } else {
                // if the leaver was the host - assign a new host
                if current_lobby.host_agent_id == leaver_agent_id {
                    let next_host_agent_id = current_lobby.agents.first().expect("another player");
                    current_lobby.host_agent_id = *next_host_agent_id;
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

    pub fn join_lobby(&mut self, agent_id: AgentId, lobby_id: LobbyId) -> Result<(), Error> {
        match self.leave_current_lobby(agent_id) {
            Ok(_) => {}
            Err(err) => {
                if let Error::AgentDoesNotExist(_) = err {
                    return Err(err);
                }
            }
        }

        let lobby = match self.get_lobby_mut(lobby_id) {
            Some(lobby) => lobby,
            None => return Err(Error::LobbyDoesNotExist(lobby_id)),
        };

        lobby.agents.push(agent_id);

        Ok(())
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

    fn next_game_id(&self) -> LobbyId {
        LobbyId(self.last_lobby_id.0 + 1)
    }

    fn next_agent_id(&self) -> AgentId {
        AgentId(self.last_agent_id.0 + 1)
    }

    pub fn lobbies(&self) -> std::slice::Iter<'_, Lobby> {
        self.lobbies.iter()
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
    use crate::coordinator::{AgentId, Coordinator};

    #[test]
    fn test_coordinator_join_leave() {
        let mut coordinator = Coordinator::new();

        let agent_id = coordinator.create_new_agent("Jim");
        println!("agent 1 {:?}", agent_id);

        let lobby_id = coordinator.create_lobby_with_host(agent_id).unwrap();
        println!("lobby_id {:?}", lobby_id);

        let agent_2_id = coordinator.create_new_agent("Pam");
        println!("agent 2 {:?}", agent_2_id);

        coordinator.join_lobby(agent_2_id, lobby_id).unwrap();

        coordinator.leave_current_lobby(agent_id).unwrap();

        for l in coordinator.lobbies() {
            println!("{:?}", l);
        }

        let agent_id = coordinator.create_new_agent("Dwight");
        coordinator.join_lobby(agent_id, lobby_id).unwrap();

        let agent_id = coordinator.create_new_agent("Larry");
        coordinator.join_lobby(agent_id, lobby_id).unwrap();

        let agent_id = coordinator.create_new_agent("Denis");
        coordinator.join_lobby(agent_id, lobby_id).unwrap();

        for l in coordinator.lobbies() {
            println!("{:?}", l);
        }

        coordinator.leave_current_lobby(AgentId(2)).unwrap();
        coordinator.leave_current_lobby(AgentId(3)).unwrap();
        coordinator.leave_current_lobby(AgentId(4)).unwrap();
        coordinator.leave_current_lobby(AgentId(5)).unwrap();

        // lobby will self-delete when last player leaves
        assert!(coordinator.join_lobby(agent_id, lobby_id).is_err());
    }

    #[test]
    fn test_coordinator_2p_game() {
        let mut coordinator = Coordinator::new();

        let agent_id = coordinator.create_new_agent("Denis");
        println!("agent 1 {:?}", agent_id);

        let lobby_id = coordinator.create_lobby_with_host(agent_id).unwrap();
        println!("lobby_id {:?}", lobby_id);

        let agent_2_id = coordinator.create_new_agent("Greg");
        println!("agent 2 {:?}", agent_2_id);

        coordinator.join_lobby(agent_2_id, lobby_id).unwrap();

        let _runner_arc_mutex = coordinator.start_game(lobby_id).unwrap();

        print!("");
    }
}