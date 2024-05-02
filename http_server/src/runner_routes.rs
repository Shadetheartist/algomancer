use std::num::ParseIntError;
use std::ops::Deref;
use std::sync::Arc;
use rocket::serde::json::Json;
use rocket::State;
use tokio::sync::RwLock;
use ws::{Message, WebSocket};
use algomanserver::{AgentId, AgentKey, Coordinator, coordinator, LobbyId, RunnerId};
use crate::{Error, models};
use crate::models::{AgentModel, AgentKeyRequest, LobbyModel, RegistrationResponse};
use crate::ws::{SendJsonError, ws_close_with_error, ws_lobby_listen, ws_send_json, ws_send_text, ws_request_response, ws_request_agent_key, RequestResponseError};
use rocket::futures::{SinkExt, StreamExt};
use algomanserver::client::ClientKey;
use crate::messages::WsEvent::AgentJoinedLobby;
use crate::messages::{WsMessage, WsRequest, WsResponse};


#[get("/connect")]
pub async fn runner_connect(ws: WebSocket, runners: &State<Arc<RwLock<Vec<algomanserver::Runner>>>>) -> ws::Channel<'static> {
    let runners = runners.inner().clone();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            println!("someone connected");
            let (mut tx, mut rx) = stream.split();

            match ws_request_response(&mut tx, &mut rx, WsRequest::MigrationInfoRequest).await {
                Ok(response) => {
                    match response {
                        WsResponse::MigrationInfoResponse { info } => {
                            println!("they have info");

                            let runner_id: RunnerId = info.runner_id.parse::<u64>().unwrap().into();

                            {
                                let mut runners = runners.write().await;
                                if let Some(runner) = runners.iter_mut().find(|r| r.runner_id == runner_id) {
                                    let client_key: ClientKey = info.client_key.parse::<u64>().unwrap().into();
                                    let mut ready_rx = runner.connect_client(client_key);

                                    println!("connected");

                                    while let Ok(_) = ready_rx.recv().await {
                                        println!("msg");
                                        break;
                                    }

                                    println!("running");


                                } else {
                                    ws_close_with_error(tx, "runner not found".to_string()).await;
                                    return Ok(());
                                }
                            }
                        }
                        _ => {
                            ws_close_with_error(tx, "expected migration info response".to_string()).await;
                            return Ok(());
                        }
                    }
                }
                Err(err) => {
                    ws_close_with_error(tx, err.to_string()).await;
                    return Ok(());
                }
            }

            Ok(())
        })
    })
}
