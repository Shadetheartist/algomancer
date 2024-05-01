use std::num::ParseIntError;
use std::ops::Deref;
use std::sync::Arc;
use rocket::serde::json::Json;
use rocket::State;
use tokio::sync::RwLock;
use ws::{Message, WebSocket};
use algomanserver::{AgentId, AgentKey, Coordinator, coordinator, LobbyId};
use crate::{Error, models};
use crate::models::{AgentModel, AgentKeyRequest, LobbyModel, RegistrationResponse};
use crate::ws::{SendJsonError, ws_close_with_error, ws_lobby_listen, ws_send_json, ws_send_text, ws_request_response, ws_request_agent_key, RequestResponseError};
use rocket::futures::{SinkExt, StreamExt};
use crate::messages::WsEvent::AgentJoinedLobby;
use crate::messages::{WsMessage, WsRequest, WsResponse};
use crate::messages::WsResponse::AgentKeyResponse;


#[get("/runner/connect")]
pub async fn runner_connect(ws: WebSocket, runners: &State<Arc<RwLock<Vec<algomanserver::Runner>>>>) -> ws::Channel<'static> {
    let runners = runners.inner().clone();

    ws.channel(move |mut stream| {
        Box::pin(async move {

            Ok(())
        })
    })
}
