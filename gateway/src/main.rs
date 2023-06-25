// Internal generated modules
use gateway::{ asset_transfer_server::{ AssetTransferServer } };

// Internal modules
use services::asset_transfer_service::AssetTransferService;

// External modules
use config;
use std::env;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use tokio::sync::RwLock;
use tonic::{ transport::Server };

pub mod gateway {
    tonic::include_proto!("gateway");
}

mod db;
mod services;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = config::Config::default();
    // Either get config path from environment variable or uses default.
    let config_file_name = env::var("GATEWAY_CONFIG").unwrap_or_else(|_| {
        println!("Using default config `config/Settings`");
        "config/Settings".to_string()
    });

    settings
        .merge(config::File::with_name(&config_file_name))
        .unwrap()
        // Add in settings from the environment (with a prefix of Gateway) Can be used to override config file settings
        .merge(config::Environment::with_prefix("GATEWAY"))
        .unwrap();

    let gateway_name = settings.get_str("name").expect("No Gateway name provided");
    println!("Gateway Name: {:?}", gateway_name);
    let gateway_port = settings
        .get_str("port")
        .expect(
            &format!(
                "Port does not exist for gateway name {}. Make sure the config file <{}> has the name and port specified.",
                gateway_name,
                config_file_name.to_string()
            )
        );
    let host = settings.get_str("hostname").unwrap_or("localhost".to_string());

    // Converts port to a valid socket address
    let address: SocketAddr = format!("{}:{}", host, gateway_port)
        .to_socket_addrs()?
        .next()
        .expect("Port number is potentially invalid. Unable to create SocketAddr");

    let asset_transfer_service = AssetTransferService {
      config_lock: RwLock::new(settings.clone()),
    };

    Server::builder()
        .add_service(AssetTransferServer::new(asset_transfer_service))
        .serve(address).await?;
    Ok(())
}
