

use std::sync::Arc;

use rocket::State;
use tokio::sync::RwLock;
use ws::{WebSocket};
use algomanserver::{RunnerId};


use crate::ws::{ws_close_with_error, ws_request_response};
use rocket::futures::{StreamExt};
use algomanserver::client::ClientKey;

use crate::messages::{ServerRequest, ClientResponse};


#[get("/connect")]
pub async fn runner_connect(ws: WebSocket, runners: &State<Arc<RwLock<Vec<algomanserver::Runner>>>>) -> ws::Channel<'static> {
    let runners = runners.inner().clone();

    ws.channel(move |stream| {
        Box::pin(async move {
            println!("someone connected");
            let (mut tx, mut rx) = stream.split();

            match ws_request_response(&mut tx, &mut rx, ServerRequest::MigrationInfoRequest).await {
                Ok(response) => {
                    match response {
                        ClientResponse::MigrationInfoResponse { info } => {
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
