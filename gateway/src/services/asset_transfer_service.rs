// Internal generated modules
use satp::{
    gateway::assettransfer::{
        asset_transfer_server::AssetTransfer,
        InitializationRequest,
        InitializationDenied,
        InitializationResponse,
    },
    common::ack::{ Ack, ack },
};

// Internal modules
use crate::{ db::Database, services::helpers::transfer_initiation_helper };

// external modules
use tonic::{ Request, Response, Status };
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct AssetTransferService {
    pub config_lock: RwLock<config::Config>,
}

/// AssetTransferService is the gRPC server implementation that handles the logic for
/// communication of the asset transfer protocol between two gateways.
#[tonic::async_trait]
impl AssetTransfer for AssetTransferService {
    async fn transfer_initiation_request(
        &self,
        request: Request<InitializationRequest>
    ) -> Result<Response<Ack>, Status> {
        println!(
            "Got a transfer initiation request from {:?} - {:?}",
            request.remote_addr(),
            request
        );

        let initialization_request = request.into_inner().clone();
        let conf = self.config_lock.read().await;

        // Database access/storage
        let db = Database {
            db_path: conf.get_str("db_path").unwrap(),
            db_open_max_retries: conf.get_int("db_open_max_retries").unwrap_or(500) as u32,
            db_open_retry_backoff_msec: conf
                .get_int("db_open_retry_backoff_msec")
                .unwrap_or(10) as u32,
        };

        match transfer_initiation_helper(db, initialization_request, conf.clone()) {
            Ok(ack) => {
                let reply = Ok(Response::new(ack));
                println!("Sending back Ack: {:?}\n", reply);
                reply
            }
            Err(e) => {
                println!("Transfer initiation failed.");
                let reply = Ok(
                    // TODO: remove the hardcoded value
                    Response::new(Ack {
                        status: ack::Status::Error as i32,
                        request_id: "xxxxxxxxx".to_string(),
                        message: format!("Error: Transfer initiation failed. {:?}", e),
                    })
                );
                println!("Sending back Ack: {:?}\n", reply);
                reply
            }
        }
    }

    async fn transfer_initiation_response(
        &self,
        request: Request<InitializationResponse>
    ) -> Result<Response<Ack>, Status> {
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

        match version.as_str() {
            "1" =>
                Ok(
                    Response::new(Ack {
                        status: ack::Status::Error as i32,
                        request_id: "xxxxxxxxx".to_string(),
                        message: "Ack of the transfer initiation response".to_string(),
                    })
                ),
            _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid version provided")),
        }
    }

    async fn transfer_initiation_denied(
        &self,
        request: Request<InitializationDenied>
    ) -> Result<Response<Ack>, Status> {
        println!(
            "Got a transfer initiation request from {:?} - {:?}",
            request.remote_addr(),
            request
        );

        let query = request.into_inner().clone();
        let version = query.reason.to_string();
        let conf = self.config_lock.read().await;
        // Database access/storage
        let db = Database {
            db_path: conf.get_str("db_path").unwrap(),
            db_open_max_retries: conf.get_int("db_open_max_retries").unwrap_or(500) as u32,
            db_open_retry_backoff_msec: conf
                .get_int("db_open_retry_backoff_msec")
                .unwrap_or(10) as u32,
        };

        match version.as_str() {
            "1" =>
                Ok(
                    Response::new(Ack {
                        status: ack::Status::Error as i32,
                        request_id: "xxxxxxxxx".to_string(),
                        message: "Ack of the transfer initiation denied".to_string(),
                    })
                ),
            _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid version provided")),
        }
    }
}
