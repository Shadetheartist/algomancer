use std::borrow::Cow;
use std::ops::DerefMut;
use std::sync::{Arc};
use rocket::futures::SinkExt;
use rocket::futures::StreamExt;
use rocket::futures::stream::{SplitSink, SplitStream};
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;
use ws::frame::CloseFrame;
use ws::Message;
use algomanserver::{AgentKey, Coordinator, LobbyId};

pub async fn ws_send_text(tx: &mut SplitSink<ws::stream::DuplexStream, Message>, msg: &str) -> Result<(), ws::result::Error> {
    tx.send(Message::Text(msg.to_string())).await
}

pub async fn ws_wait_for<T: DeserializeOwned>(what: &str, tx: &mut SplitSink<ws::stream::DuplexStream, Message>, rx: &mut SplitStream<ws::stream::DuplexStream>) -> Option<T> {
    if ws_send_text(tx, format!("Waiting for {what}").as_str()).await.is_err() {
        return None;
    }

    while let Some(message) = rx.next().await {
        if let Ok(message) = message {
            match message {
                Message::Text(text) => {
                    match serde_json::from_str::<T>(&text) {
                        Ok(value) => {
                            return Some(value);
                        }
                        Err(err) => {
                            match tx.send(Message::Text(format!("Error deserializing message: {err}. (Still Waiting for {what})").to_string())).await {
                                Ok(_) => {}
                                Err(send_err) => {
                                    eprintln!("{}", send_err);
                                    return None;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    None
}


pub async fn ws_close_normally(tx: &mut SplitSink<ws::stream::DuplexStream, Message>) {
    tx.send(Message::Close(Some(CloseFrame{
        code: ws::frame::CloseCode::Normal,
        reason: Default::default()
    }))).await.ok();
}

pub async fn ws_close_with_error(mut tx: SplitSink<ws::stream::DuplexStream, Message>, err_msg: String) {
    tx.send(Message::Text(err_msg.to_string())).await.ok();
    tx.send(Message::Close(Some(CloseFrame{
        code: ws::frame::CloseCode::Normal,
        reason: Cow::from(err_msg)
    }))).await.ok();
}


pub async fn ws_lobby_listen(
    coordinator: Arc<RwLock<Coordinator>>,
    agent_key: AgentKey,
    lobby_id: LobbyId,
    mut tx: SplitSink<ws::stream::DuplexStream, Message>,
    mut rx: SplitStream<ws::stream::DuplexStream>)
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