use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::algomancer;

use crate::algomancer::{ConnectRequest, ConnectResponse};

#[derive(Debug)]
pub struct CoordinatorService {
    pub inner: Arc<tokio::sync::RwLock<crate::coordinator::Coordinator>>
}

impl CoordinatorService {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(tokio::sync::RwLock::new(crate::coordinator::Coordinator::new())),
        }
    }
}


#[async_trait]
impl algomancer::coordinator_server::Coordinator for CoordinatorService {
    async fn register(&self, request: Request<ConnectRequest>) -> Result<Response<ConnectResponse>, Status> {

        let request = request.get_ref();

        let agent_id = {
            let mut coordinator = self.inner.write().await;
            coordinator.create_new_agent(request.username.as_str())
        };

        let response: Response<ConnectResponse> = Response::new(ConnectResponse {
            agent_id: agent_id.0 as i64
        });


        Ok(response)
    }
}