use rocket::futures::SinkExt;
use rocket::futures::StreamExt;
use rocket::futures::stream::{SplitSink, SplitStream};
use serde::de::DeserializeOwned;
use ws::Message;

pub async fn ws_wait_for<T: DeserializeOwned>(what: &str, tx: &mut SplitSink<ws::stream::DuplexStream, Message>, rx: &mut SplitStream<ws::stream::DuplexStream>) -> Option<T> {
    if tx.send(Message::Text(format!("Waiting for {what}").to_string())).await.is_err() {
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

