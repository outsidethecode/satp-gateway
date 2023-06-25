// Internal generated modules
use crate::gateway::{
    asset_transfer_server::AssetTransfer,
    InitializationRequest,
    InitializationResponse,
};

// external modules
use tonic::{ Request, Response, Status };

#[derive(Debug, Default)]
pub struct AssetTransferService {}

#[tonic::async_trait]
impl AssetTransfer for AssetTransferService {
    async fn transfer_initiation(
        &self,
        request: Request<InitializationRequest>
    ) -> Result<Response<InitializationResponse>, Status> {
        println!(
            "Got a transfer initiation request from {:?} - {:?}",
            request.remote_addr(),
            request
        );

        let r = request.into_inner();
        match r.version.as_str() {
            "1" =>
                Ok(
                    Response::new(InitializationResponse {
                        version: "version1".to_string(),
                        message_type: "message_type1".to_string(),
                        session_id: "session_id1".to_string(),
                        transfer_context_id: "transfer_context_id1".to_string(),
                        hash_transfer_init_claims: "hash_transfer_init_claims1".to_string(),
                        timestamp: "timestamp1".to_string(),
                    })
                ),
            _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid version provided")),
        }
    }
}
