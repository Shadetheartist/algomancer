use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use algomancer_gre::game::GameOptions;
use crate::coordinator::agent::AgentId;
use crate::Error;
use crate::Error::{NotListening, SendEventError};
use crate::runner::MigrationInfo;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct LobbyId(pub u64);

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
pub struct Lobby {
    pub id: LobbyId,

    pub options: GameOptions,

    pub host_agent_id: AgentId,
    pub agent_ids: Vec<AgentId>,

    pub event_sender: HashMap<AgentId, tokio::sync::mpsc::Sender<LobbyEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum LobbyEvent {
    AgentJoined(AgentId),
    AgentLeft(AgentId),
    NewHost(AgentId),

    Migrate(AgentId, MigrationInfo),
    Whisper(AgentId, AgentId, String)
}


impl Lobby {
    pub async fn send_event(&self, lobby_event: LobbyEvent) -> Result<(), Error>{
        match lobby_event {
            LobbyEvent::AgentJoined(_) |
            LobbyEvent::AgentLeft(_) |
            LobbyEvent::NewHost(_) => {
                println!("queuing broadcast of event to all lobby target channels: {:?}", lobby_event);
                for (_, rx) in &self.event_sender {
                    match rx.send(lobby_event.clone()).await {
                        Ok(_) => {}
                        Err(err) => {
                            return Err(SendEventError(err));
                        }
                    }
                }

                Ok(())
            }

            LobbyEvent::Migrate(agent_id, _) |
            LobbyEvent::Whisper(_, agent_id, _) => {
                let target_tx = match self.event_sender.get(&agent_id) {
                    None => {
                        return Err(NotListening(agent_id));
                    }
                    Some(target_tx) => target_tx
                };

                match target_tx.send(lobby_event.clone()).await {
                    Ok(_) => {
                        println!("queued targeted message to channel: {:?}", lobby_event);
                        Ok(())
                    }
                    Err(err) => {
                        return Err(SendEventError(err));
                    }
                }


            }
        }
    }
}