use tonic::transport::Server;
use tonic::{Request, Response, Status};

// Import the generated proto code
mod proto {
    pub mod parflow {
        tonic::include_proto!("parflow");
    }
}
use proto::parflow::orchestrator_server::{Orchestrator, OrchestratorServer};
use proto::parflow::{OrchestratorRequest, OrchestratorResponse};

#[derive(Default)]
pub struct MyOrchestrator {}

#[tonic::async_trait]
impl Orchestrator for MyOrchestrator {
    async fn run(
        &self,
        _request: Request<OrchestratorRequest>,
    ) -> Result<Response<OrchestratorResponse>, Status> {
        // call core example (parallel)
        let results = parflow_core::run_example_par().await;
        let reply = OrchestratorResponse { results };
        Ok(Response::new(reply))
    }
}

pub async fn run_grpc_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("[::1]:{}", port).parse()?;
    println!("🔌 gRPC server listening on {}", addr);
    Server::builder()
        .add_service(OrchestratorServer::new(MyOrchestrator::default()))
        .serve(addr)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting ParFlow gRPC Server");

    let port = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(50051);

    run_grpc_server(port).await
}
