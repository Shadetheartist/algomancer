use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::ops::DerefMut;
use std::sync::{Arc};
use rocket::futures::SinkExt;
use rocket::futures::StreamExt;
use rocket::futures::stream::{SplitSink, SplitStream};
use serde::Serialize;
use tokio::sync::RwLock;
use ws::frame::CloseFrame;
use ws::Message;
use algomanserver::{AgentKey, Coordinator, LobbyId};
use algomanserver::coordinator::Error;
use crate::messages::{WsMessage, WsRequest, WsResponse};
use crate::messages::WsResponse::AgentKeyResponse;


type TX = SplitSink<ws::stream::DuplexStream, Message>;
type RX = SplitStream<ws::stream::DuplexStream>;


pub async fn ws_request_agent_key(tx: &mut TX, rx: &mut RX) -> Result<AgentKey, RequestResponseError> {
    let agent_key_request = WsRequest::AgentKeyRequest;
    let ws_response = match ws_request_response(tx, rx, agent_key_request).await {
        Ok(response) => response,
        Err(err) => return Err(err)
    };

    let agent_key = {
        if let AgentKeyResponse { agent_key } = ws_response {
            let agent_key: AgentKey = match agent_key.parse::<u64>() {
                Ok(agent_key) => agent_key.into(),
                Err(_) => return Err(RequestResponseError::InvalidResponse)
            };

            agent_key
        } else {
            return Err(RequestResponseError::InvalidResponse);
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
    InvalidResponse,
    ConnectionClosed,
    MessageNotText,
}

impl Display for RequestResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestResponseError::ErrorSendingJson(err) => write!(f, "error sending json: {err}"),
            RequestResponseError::ErrorDeserializingMessage(err) => write!(f, "error deserializing message: {err}"),
            RequestResponseError::InvalidResponse => write!(f, "received invalid response"),
            RequestResponseError::ConnectionClosed => write!(f, "connection has been closed"),
            RequestResponseError::MessageNotText => write!(f, "received message was not a text frame"),
        }
    }
}

pub async fn ws_request_response(tx: &mut TX, rx: &mut RX, request_variant: WsRequest) -> Result<WsResponse, RequestResponseError> {
    if let Err(err) = ws_send_json(tx, &WsMessage::Request { value: request_variant }).await {
        return Err(RequestResponseError::ErrorSendingJson(err));
    }

    while let Some(message) = rx.next().await {
        if let Ok(message) = message {
            return match message {
                Message::Text(text) => {
                    match serde_json::from_str::<WsMessage>(&text) {
                        Ok(value) => {
                            match value {
                                WsMessage::Response { value } => Ok(value),
                                _ => Err(RequestResponseError::InvalidResponse)
                            }
                        }
                        Err(err) => {
                            Err(RequestResponseError::ErrorDeserializingMessage(err))
                        }
                    }
                }
                _ => Err(RequestResponseError::MessageNotText)
            };
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
    tx.send(Message::Text(err_msg.to_string())).await.ok();
    tx.send(Message::Close(Some(CloseFrame {
        code: ws::frame::CloseCode::Normal,
        reason: Cow::from(err_msg),
    }))).await.ok();
}


pub async fn ws_lobby_listen(
    coordinator: Arc<RwLock<Coordinator>>,
    agent_key: AgentKey,
    lobby_id: LobbyId,
    mut tx: TX,
    mut rx: RX)
{
    let mut lobby_rx = {
        let mut coordinator = coordinator.write().await;
        match coordinator.lobby_listen(agent_key, lobby_id.into()) {
            Ok(lobby_rx) => lobby_rx,
            Err(err) => {
                ws_close_with_error(tx, format!("{}", err)).await;
                return;
            }
        }
    };

    let tx = Arc::new(tokio::sync::Mutex::new(tx));

    let mut send_task = {
        let tx = tx.clone();

        tokio::spawn(async move {
            while let Some(lobby_event) = lobby_rx.recv().await {
                let event_json = serde_json::to_string(&lobby_event).expect("serialized lobby event");
                tx.lock().await.send(Message::Text(event_json)).await.ok();
            }
        })
    };

    let mut recv_task = tokio::spawn(async move {
        while let Some(message) = rx.next().await {
            if let Ok(message) = message {
                match message {
                    Message::Text(_) => {
                        tx.lock().await.send(Message::Text("EEE".to_string())).await.ok();
                    }
                    Message::Binary(_) => {}
                    Message::Ping(_) => {}
                    Message::Pong(_) => {}
                    Message::Close(_) => {
                        ws_close_normally(tx.lock().await.deref_mut()).await;
                        return;
                    }
                    Message::Frame(_) => {}
                }
                println!("received message")
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
