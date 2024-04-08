mod coordinator;

use std::net::SocketAddr;
use game_service::game_server::{Game, GameServer};
use game_service::{GetStateRequest, GetStateResponse};
use game_service::{ApplyActionRequest, ApplyActionResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod game_service {
    tonic::include_proto!("game_service");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("game_service_descriptor");
}

#[derive(Default)]
pub struct MyGame {}

#[tonic::async_trait]
impl Game for MyGame {
    async fn get_state(&self, _request: Request<GetStateRequest>) -> Result<Response<GetStateResponse>, Status> {
        let _input = _request.get_ref();

        println!("get state");
        Ok(Response::new(GetStateResponse {}))
    }
    
    async fn apply_action(&self, _request: Request<ApplyActionRequest>) -> Result<Response<ApplyActionResponse>, Status> {
        println!("apply action");
        Ok(Response::new(ApplyActionResponse {}))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:8000".parse()?;
    let my_game = MyGame::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(game_service::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(GameServer::new(my_game))
        .serve(addr)
        .await?;

    Ok(())
}
