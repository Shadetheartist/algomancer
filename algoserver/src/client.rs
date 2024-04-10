use game_service::game_client::GameClient;
use game_service::GetStateRequest;

pub mod game_service {
    tonic::include_proto!("game_service");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut my_client = GameClient::connect("http://127.0.0.1:8000").await?;

    let request = tonic::Request::new(GetStateRequest{});

    let response = my_game_client.get_state(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
