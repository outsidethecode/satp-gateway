// Internal generated modules
use satp::{
    gateway::assettransfer::{ asset_transfer_server::AssetTransfer, InitializationRequest, InitializationDenied, InitializationResponse },
    common::ack::{Ack, ack},
};

// Internal modules
use crate::{error::Error, db::Database};

// external modules
use tonic::{ Request, Response, Status };
use tokio::sync::RwLock;

use super::types::{ Network, Driver };

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

/// transfer_initiation_helper is run on the remote gateway to initiate asset transfer protocol that was
/// requested from the requesting gateway
fn transfer_initiation_helper(
    db: Database,
    initialization_request: InitializationRequest,
    conf: config::Config
) -> Result<Ack, Error> {
    let _set_query = db
        .set(&initialization_request.session_id.to_string(), &initialization_request.session_id)
        .map_err(|e| Error::Simple(format!("DB Failure: {:?}", e)))?;

    //TODO remove hardcoded value
    let network_id = "Dummy_Network";
    let result = get_driver(network_id.to_string(), conf.clone());
    match result {
        Ok(driver_info) => {
            // spawn_request_driver_state(query, driver_info, conf.clone());
            // TODO: remove the hardcoded value
            return Ok(Ack {
                status: ack::Status::Ok as i32,
                request_id: "xxxxxxxx".to_string(),
                message: driver_info.hostname.to_string(),
            });
        }
        Err(e) => Err(e),
    }
}

pub fn get_driver(network_id: String, conf: config::Config) -> Result<Driver, Error> {
    // get the driver type from the networks map
    let networks_table = conf
        .get_table("networks")
        .map_err(|e| Error::Simple(format!("Unable to find networks table. Error: {:?}", e)))?;
    let network_table = networks_table
        .get::<String>(&network_id.to_string())
        .ok_or(
            Error::Simple(
                format!("Unable to find Network_id \"{}\" in config", network_id.to_string())
            )
        )?;
    let network_type = network_table
        .clone()
        .try_into::<Network>()
        .expect("Error in config file networks table")
        .clone();
    // get the driver host:port from the drivers map
    let drivers_table = conf
        .get_table("drivers")
        .map_err(|e| Error::Simple(format!("Unable to find driver table. Error: {:?}", e)))?;
    let driver_table = drivers_table
        .get::<String>(&network_type.network)
        .ok_or(
            Error::Simple(
                format!("Unable to find driver port for network: {}", network_id.to_string())
            )
        )?;
    let driver_info = driver_table
        .clone()
        .try_into::<Driver>()
        .expect("Error in config file drivers table");
    return Ok(driver_info);
}
