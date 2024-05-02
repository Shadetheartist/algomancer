use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use algomacros::impl_u64_key_wrapper;
use algomancer_gre::game::GameOptions;
use crate::AgentKey;
use crate::coordinator::agent::AgentId;
use crate::coordinator::Error;
use crate::coordinator::Error::{NotListening, SendEventError};
use crate::runner::MigrationInfo;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LobbyId(pub u64);
impl_u64_key_wrapper!(LobbyId);

#[derive(Debug)]
pub struct Lobby {
    pub id: LobbyId,
    pub capacity: u8,
    pub host_agent_id: AgentId,
    pub name: String,

    pub game_options: GameOptions,

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
            // publicly broadcasted events
            LobbyEvent::AgentJoined(_) |
            LobbyEvent::AgentLeft(_) |
            LobbyEvent::NewHost(_) => {
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

            // privately sent events
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