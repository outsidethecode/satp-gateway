// Internal generated modules
use crate::{
    gateway::{
        asset_transfer_server::AssetTransfer,
        InitializationRequest,
        InitializationResponse,
    },
    db::Database,
};

// Internal modules
use crate::error::Error;

// external modules
use tonic::{ Request, Response, Status };
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct AssetTransferService {
    pub config_lock: RwLock<config::Config>,
}

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

        let query = request.into_inner().clone();
        let version = query.version.to_string();
        let conf = self.config_lock.read().await;
        // Database access/storage
        let db = Database {
            db_path: conf.get_str("db_path").unwrap(),
            db_open_max_retries: conf.get_int("db_open_max_retries").unwrap_or(500) as u32,
            db_open_retry_backoff_msec: conf
                .get_int("db_open_retry_backoff_msec")
                .unwrap_or(10) as u32,
        };

        let logged = log_request(db, query);

        match version.as_str() {
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

fn log_request (
    db: Database,
    query: InitializationRequest,
) -> Result<(), Error> {
    let _set_query = db
        .set(&query.session_id.to_string(), &query.session_id)
        .map_err(|e| Error::Simple(format!("DB Failure: {:?}", e)))?;
    
    return Ok(());
}