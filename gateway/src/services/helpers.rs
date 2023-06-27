// Internal generated modules
use satp::{ gateway::assettransfer::{ InitializationRequest }, common::ack::{ Ack, ack } };

// Internal modules
use crate::{ error::Error, db::Database };
use super::types::{ Network, Driver };

/// transfer_initiation_helper is run on the remote gateway to initiate asset transfer protocol that was
/// requested from the requesting gateway
pub fn transfer_initiation_helper(
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
            spawn_check_transfer_inititation_request(
                initialization_request,
                &driver_info,
                conf.clone()
            );
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

// Function that starts a thread which checks the transfer initialization request
fn spawn_check_transfer_inititation_request(
    initialization_request: InitializationRequest,
    driver_info: &Driver,
    conf: config::Config
) {
    tokio::spawn(async move {
        let is_valid_request = check_transfer_inititation_request(initialization_request.clone());
        if is_valid_request {
            // Send an InitiationResponse message to the requesting gateway
            println!("The asset initiation request is valid\n");
        } else {
            // Send an InitiationDenied message to the requesting gateway
            println!("The asset initiation request is denied\n");
        }
    });
}

fn check_transfer_inititation_request(initialization_request: InitializationRequest) -> bool {
    true
}
