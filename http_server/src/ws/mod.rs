use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc};
use std::time::Duration;
use rocket::futures::SinkExt;
use rocket::futures::StreamExt;
use rocket::futures::stream::{SplitSink, SplitStream};
use serde::Serialize;
use tokio::sync::RwLock;
use tokio::time::timeout;
use ws::frame::CloseFrame;
use ws::Message;
use algomanserver::{AgentKey, Coordinator, LobbyEvent, LobbyId, Runner};
use crate::messages::{ServerEvent, WsMessage, ServerRequest, ClientResponse, ClientRequest};
use crate::models::{AgentModel, LobbyModel, MigrationInfoModel};


type TX = SplitSink<ws::stream::DuplexStream, Message>;
type RX = SplitStream<ws::stream::DuplexStream>;


fn default_timeout_duration() -> Duration {
    Duration::from_secs(1)
}

pub async fn ws_request_agent_key(tx: &mut TX, rx: &mut RX) -> Result<AgentKey, RequestResponseError> {
    let agent_key_request = ServerRequest::AgentKeyRequest;

    let ws_response = match ws_request_response_within(tx, rx, agent_key_request, default_timeout_duration()).await {
        Ok(response) => response,
        Err(err) => return Err(err)
    };

    let agent_key = {
        if let ClientResponse::AgentKeyResponse { agent_key } = ws_response {
            let agent_key: AgentKey = match agent_key.parse() {
                Ok(agent_key) => agent_key,
                Err(_) => return Err(RequestResponseError::InvalidResponse("could not parse agent key".to_string()))
            };

            agent_key
        } else {
            return Err(RequestResponseError::InvalidResponse("expected agent key response".to_string()));
        }
    };

    Ok(agent_key)
}

pub enum SendJsonError {
    WebsocketError(ws::result::Error),
    SerializationError(serde_json::Error),
}

impl Display for SendJsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SendJsonError::WebsocketError(err) => write!(f, "websocket error when sending json: {:?}", err),
            SendJsonError::SerializationError(err) => write!(f, "serialization error when sending json: {:?}", err)
        }
    }
}

pub async fn ws_send_text(tx: &mut TX, msg: &str) -> Result<(), ws::result::Error> {
    tx.send(Message::Text(msg.to_string())).await
}

pub async fn ws_send_json<T: Serialize>(tx: &mut TX, var: &T) -> Result<(), SendJsonError> {
    let json = match serde_json::to_string(var) {
        Ok(json) => json,
        Err(err) => {
            return Err(SendJsonError::SerializationError(err));
        }
    };

    println!("sending json: {json}");

    if let Err(err) = ws_send_text(tx, json.as_str()).await {
        return Err(SendJsonError::WebsocketError(err));
    }

    Ok(())
}

pub enum RequestResponseError {
    ErrorSendingJson(SendJsonError),
    ErrorDeserializingMessage(serde_json::Error),
    InvalidResponse(String),
    InvalidRequest(String),
    ErrorStartingGame(String),
    ConnectionClosed,
    MessageNotText,
    Timeout,
}

impl Display for RequestResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestResponseError::ErrorSendingJson(err) => write!(f, "error sending json: {err}"),
            RequestResponseError::ErrorDeserializingMessage(err) => write!(f, "error deserializing message: {err}"),
            RequestResponseError::InvalidResponse(reason) => write!(f, "received invalid response: {reason}"),
            RequestResponseError::ConnectionClosed => write!(f, "connection has been closed"),
            RequestResponseError::MessageNotText => write!(f, "received message was not a text frame"),
            RequestResponseError::InvalidRequest(reason) => write!(f, "received an invalid request: {reason}"),
            RequestResponseError::ErrorStartingGame(reason) => write!(f, "error starting game: {reason}"),
            RequestResponseError::Timeout => write!(f, "client did not send response within the allotted timeframe"),
        }
    }
}

pub async fn ws_send_err(tx: &mut TX, err_str: String) -> Result<(), SendJsonError> {
    ws_send_json(tx, &WsMessage::Error { value: err_str }).await
}

pub async fn ws_request_response_within(tx: &mut TX, rx: &mut RX, request_variant: ServerRequest, duration: Duration) -> Result<ClientResponse, RequestResponseError> {
    let result = timeout(duration, ws_request_response(tx, rx, request_variant)).await;
    result.unwrap_or_else(|_| Err(RequestResponseError::Timeout))
}

pub async fn ws_request_response(tx: &mut TX, rx: &mut RX, request_variant: ServerRequest) -> Result<ClientResponse, RequestResponseError> {
    if let Err(err) = ws_send_json(tx, &WsMessage::ServerRequest { value: request_variant.clone() }).await {
        return Err(RequestResponseError::ErrorSendingJson(err));
    }

    while let Some(message) = rx.next().await {
        if let Ok(message) = message {
            println!("received: {message}");
            let result = match message {
                Message::Text(text) => {
                    match serde_json::from_str::<WsMessage>(&text) {
                        Ok(value) => {
                            match value {
                                WsMessage::ClientResponse { value } => {
                                    if request_variant.is_correct_response_type(&value) {
                                        Ok(value)
                                    } else {
                                        Err(RequestResponseError::InvalidResponse("client response type does not pair with the request type".to_string()))
                                    }
                                },
                                _ => Err(RequestResponseError::InvalidResponse("expected a message with type client_response".to_string()))
                            }
                        }
                        Err(err) => {
                            Err(RequestResponseError::ErrorDeserializingMessage(err))
                        }
                    }
                }
                _ => Err(RequestResponseError::MessageNotText)
            };

            if let Err(err) = &result {
                ws_send_err(tx, err.to_string()).await.ok();
            }

            return result
        }
    }

    Err(RequestResponseError::ConnectionClosed)
}


pub async fn ws_close_normally(tx: &mut TX) {
    tx.send(Message::Close(Some(CloseFrame {
        code: ws::frame::CloseCode::Normal,
        reason: Default::default(),
    }))).await.ok();
}

pub async fn ws_close_with_error(mut tx: TX, err_msg: String) {
    eprintln!("{err_msg}");
    tx.send(Message::Text(err_msg.to_string())).await.ok();
    tx.send(Message::Close(Some(CloseFrame {
        code: ws::frame::CloseCode::Normal,
        reason: Cow::from(err_msg),
    }))).await.ok();
}


async fn respond_to_client_request(text: &str, _tx: &mut TX, runners: &mut Vec<Runner>, coordinator: &mut Coordinator) -> Result<(), RequestResponseError> {
    let request = match serde_json::from_str::<WsMessage>(text) {
        Ok(value) => {
            match value {
                WsMessage::ClientRequest { value } => value,
                _ => return Err(RequestResponseError::InvalidResponse("expected a message with type request".to_string()))
            }
        }
        Err(err) => {
            return Err(RequestResponseError::ErrorDeserializingMessage(err));
        }
    };

    match request {
        ClientRequest::StartGameRequest { agent_key, lobby_id } => {
            let agent_key: AgentKey = match agent_key.parse::<u64>() {
                Ok(agent_key) => agent_key.into(),
                Err(_) => return Err(RequestResponseError::InvalidRequest("failed to parse agent key".to_string()))
            };

            let lobby_id: LobbyId = match lobby_id.parse::<u64>() {
                Ok(lobby_id) => lobby_id.into(),
                Err(_) => return Err(RequestResponseError::InvalidRequest("failed to parse lobby id".to_string()))
            };

            match coordinator.start_game(agent_key, lobby_id).await {
                Ok(runner) => {
                    runners.push(runner)
                }
                Err(err) => return Err(RequestResponseError::ErrorStartingGame(format!("{:?}", err)))
            }

            Ok(())
        }
    }
}

pub async fn ws_lobby_listen(
    runners: Arc<RwLock<Vec<algomanserver::Runner>>>,
    coordinator: Arc<RwLock<Coordinator>>,
    agent_key: AgentKey,
    lobby_id: LobbyId,
    tx: TX,
    mut rx: RX)
{
    let mut lobby_rx = {
        let mut coordinator = coordinator.write().await;
        match coordinator.lobby_listen(agent_key, lobby_id) {
            Ok(lobby_rx) => lobby_rx,
            Err(err) => {
                ws_close_with_error(tx, format!("{}", err)).await;
                return;
            }
        }
    };

    let tx = Arc::new(tokio::sync::Mutex::new(tx));

    let mut send_task = {
        let coordinator = coordinator.clone();
        {
            let tx = tx.clone();

            tokio::spawn(async move {
                while let Some(lobby_event) = lobby_rx.recv().await {
                    let event = match lobby_event {
                        LobbyEvent::Migrate(agent_id, migration_info) => ServerEvent::Migrate {
                            agent_id: agent_id.to_string(),
                            migration_info: MigrationInfoModel {
                                runner_id: migration_info.runner_id.to_string(),
                                agent_key: migration_info.agent_key.to_string(),
                                client_key: migration_info.client_key.to_string(),
                            },
                        },
                        LobbyEvent::AgentJoined(_agent_id) => {
                            let coordinator = coordinator.read().await;
                            let agent: AgentModel = coordinator.get_agent_by_key(agent_key).expect("an agent").into();
                            let lobby = LobbyModel::from_coordinator_lobby(coordinator.deref(), coordinator.try_get_lobby(lobby_id).expect("a lobby"));
                            ServerEvent::AgentJoinedLobby {
                                agent,
                                lobby,
                            }
                        }
                        LobbyEvent::AgentLeft(agent_id) => ServerEvent::AgentLeftLobby {
                            agent_id: agent_id.to_string(),
                        },
                        LobbyEvent::NewHost(agent_id) => ServerEvent::AgentLeftLobby {
                            agent_id: agent_id.to_string(),
                        },
                        LobbyEvent::Whisper(_, _, _) => unimplemented!(),
                    };


                    match ws_send_json(tx.lock().await.deref_mut(), &WsMessage::ServerEvent { value: event }).await {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("{err}")
                        }
                    }
                }
            })
        }
    };

    let mut recv_task = tokio::spawn(async move {
        while let Some(message) = rx.next().await {
            if let Ok(message) = message {
                match message {
                    Message::Text(text) => {
                        let mut coordinator = coordinator.write().await;
                        let mut runners = runners.write().await;
                        let mut tx = tx.lock().await;
                        match respond_to_client_request(text.as_str(), tx.deref_mut(), runners.deref_mut(), coordinator.deref_mut()).await {
                            Ok(_) => {}
                            Err(err) => {
                                eprintln!("{err}");
                                ws_send_err(tx.deref_mut(), err.to_string()).await.ok();
                            }
                        }
                    }
                    Message::Binary(_) => {}
                    Message::Ping(_) => {}
                    Message::Pong(_) => {}
                    Message::Close(_) => {
                        let mut tx = tx.lock().await;
                        ws_close_normally(tx.deref_mut()).await;
                        return;
                    }
                    Message::Frame(_) => {}
                }
            }
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
}
