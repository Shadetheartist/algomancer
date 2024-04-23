use std::sync::Arc;
use rocket::serde::json::Json;
use rocket::State;
use tokio::sync::RwLock;
use ws::{Message, WebSocket};
use algomanserver::{Coordinator, RunnerId};
use crate::{Error, models};
use crate::models::{AgentKeyRequest, RegistrationResponse};
use crate::ws_helpers::ws_wait_for;
use rocket::futures::{SinkExt, StreamExt};


#[get("/game/<runner_id>/join")]
pub async fn join_game(ws: WebSocket, coordinator: &State<Arc<RwLock<Coordinator>>>, runner_id: u64) -> ws::Channel<'static> {
    let runner_id : RunnerId = runner_id.into();



    ws.channel(move |mut stream| {
        Box::pin(async move {
            let (mut tx, mut rx) = stream.split();

            let agent_key = match ws_wait_for::<AgentKeyRequest>("agent key", &mut tx, &mut rx).await {
                None => return Ok(()),
                Some(model) => model.agent_key
            };

            Ok(())
        })
    })
}
