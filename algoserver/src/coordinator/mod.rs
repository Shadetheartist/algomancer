use algomancer_gre::game::{Game, GameOptions};
use algomancer_gre::game::game_builder::NewGameError;
use algomancer_gre::game::state::GameMode;
use algomancer_gre::game::state::player::PlayerId;
use algomancer_gre::game::state::rng::AlgomancerRngSeed;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AgentId(usize);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct LobbyId(usize);

#[derive(Debug)]
pub enum Error {
    AgentDoesNotExist(AgentId),
    LobbyDoesNotExist(LobbyId),
    NewGameError(NewGameError),
    AgentNotInLobby(AgentId),
}

#[derive(Debug)]
pub struct Controller {
    agent_id: AgentId,
    player_id: Option<PlayerId>,
}

#[derive(Debug)]
pub struct Lobby {
    id: LobbyId,

    options: GameOptions,
    game: Option<Game>,

    host_agent_id: AgentId,
    controllers: Vec<Controller>,
}

pub struct Coordinator {
    last_agent_id: AgentId,
    last_lobby_id: LobbyId,
    agents: Vec<Agent>,
    lobbies: Vec<Lobby>,
}

pub struct Agent {
    id: AgentId
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

    pub fn create_new_agent(&mut self) -> AgentId {
        let id = self.next_agent_id();
        let agent = Agent {
            id: id,
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
            options: options,
            game: None,
            host_agent_id: agent.id,
            controllers: vec![
                Controller{
                    agent_id: host_agent_id,
                    player_id: None
                }],
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

        if let Some(mut current_lobby) = self.get_current_lobby_mut(leaver_agent_id) {
            let controller_idx = current_lobby.controllers
                .iter()
                .enumerate()
                .find(|(_, c)| c.agent_id == leaver_agent_id)
                .expect(format!("a controller with an agent with id {:?}", leaver_agent_id).as_str()).0;

            current_lobby.controllers.remove(controller_idx);

            if current_lobby.controllers.len() == 0 {
                // if the lobby is empty, remove the lobby (after borrow is over)
                remove_lobby = Some(current_lobby.id);
            } else {
                // if the leaver was the host - assign a new host
                if current_lobby.host_agent_id == leaver_agent_id {
                    let next_host_agent_id = current_lobby.controllers.first().expect("another player").agent_id;
                    current_lobby.host_agent_id = next_host_agent_id;
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

        lobby.controllers.push(Controller { agent_id, player_id: None });

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
            .find(|l| l.controllers.iter().any(|c| c.agent_id == agent_id))
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

}

#[cfg(test)]
mod tests {
    use crate::coordinator::Coordinator;

    #[test]
    fn test_coordinator() {
        let mut coordinator = Coordinator::new();

        let agent_id = coordinator.create_new_agent();
        println!("agent 1 {:?}", agent_id);

        let lobby_id = coordinator.create_lobby_with_host(agent_id).unwrap();
        println!("lobby_id {:?}", lobby_id);

        let agent_2_id = coordinator.create_new_agent();
        println!("agent 2 {:?}", agent_2_id);

        coordinator.join_lobby(agent_2_id, lobby_id).unwrap();

        coordinator.leave_current_lobby(agent_id).unwrap();

        for l in coordinator.lobbies() {
            print!("{:?}", l);
        }


    }
}