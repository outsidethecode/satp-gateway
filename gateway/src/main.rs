use tonic::{transport::Server, Request, Response, Status};
use gateway::{InitializationRequest, InitializationResponse, gateway_server::{Gateway, GatewayServer}};

pub mod gateway {
  tonic::include_proto!("gateway");
}

#[derive(Debug, Default)]
pub struct GatewayService {}

#[tonic::async_trait]
impl Gateway for GatewayService {
  async fn transfer_initiation(&self, request: Request<InitializationRequest>) -> Result<Response<InitializationResponse>, Status> {
    let r = request.into_inner();
    match r.version.as_str() {
      "1" => Ok(Response::new(gateway::InitializationResponse {
        version: "version1".to_string(),
        message_type: "message_type1".to_string(),
        session_id: "session_id1".to_string(),
        transfer_context_id: "transfer_context_id1".to_string(),
        hash_transfer_init_claims: "hash_transfer_init_claims1".to_string(),
        timestamp: "timestamp1".to_string()
      })), 
      _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid version provided"))
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let address = "127.0.0.1:5000".parse().unwrap();
  let transfer_initiation_service = GatewayService::default();

  Server::builder().add_service(GatewayServer::new(transfer_initiation_service))
    .serve(address)
    .await?;
  Ok(())
}

