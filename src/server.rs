extern crate lib;

use lib::{
    configuration, kms::key_management_service_server::KeyManagementServiceServer,
    utilities::socket::create_unix_socket, vault,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vault_config = configuration::vault();
    let socket_config = configuration::socket();
    let vault_kms_server = vault::VaultKmsServer::new(
        &vault_config.vault_transit_key,
        &vault_config.vault_address,
        &vault_config.vault_token,
    );
    println!(
        "Server listening to socket @\"{}\", connecting to vault @\"{}\"",
        socket_config.socket_path, vault_config.vault_address
    );
    Server::builder()
        .add_service(KeyManagementServiceServer::new(vault_kms_server))
        .serve_with_incoming(create_unix_socket(
            &socket_config.socket_path,
            socket_config.permissions,
        )?)
        .await?;
    Ok(())
}