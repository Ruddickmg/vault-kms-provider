extern crate lib;

use std::fs;
use lib::{
    configuration, kms::key_management_service_server::KeyManagementServiceServer,
    utilities::socket::create_unix_socket, vault,
};
use tokio::join;
use tonic::transport::Server;
use lib::utilities::token;

mod checks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vault_config = configuration::vault();
    let socket_config = configuration::socket();
    let vault_kms_server =
        vault::VaultKmsServer::new(&vault_config.vault_transit_key, &vault_config.vault_address);
    let token = token::auth_token();
    println!("Auth token: {:?}", token);
    let paths = fs::read_dir("/var/run/secrets/kubernetes.io/serviceaccount").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    println!(
        "Server listening to socket @\"{}\", connecting to vault @\"{}\"",
        socket_config.socket_path, vault_config.vault_address
    );
    let (server, health_checks) = join!(
        Server::builder()
            .add_service(KeyManagementServiceServer::new(vault_kms_server))
            .serve_with_incoming(create_unix_socket(
                &socket_config.socket_path,
                socket_config.permissions,
            )?),
        checks::serve()
    );
    server?;
    health_checks?;
    Ok(())
}
