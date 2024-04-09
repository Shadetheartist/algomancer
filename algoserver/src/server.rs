mod coordinator;
mod runner;

use std::net::SocketAddr;
use tonic::{transport::Server};

pub mod algomancer {
    tonic::include_proto!("algomancer");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("algomancer_descriptor");
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:8000".parse()?;

    let coordinator_service = coordinator::service::CoordinatorService::new();

    let coordinator_service = algomancer::coordinator_server::CoordinatorServer::new(coordinator_service);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(algomancer::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(coordinator_service)
        .serve(addr)
        .await?;

    Ok(())
}
