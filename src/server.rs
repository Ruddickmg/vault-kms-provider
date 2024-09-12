extern crate lib;

use lib::{
    configuration::{socket::SocketConfiguration, vault::VaultConfiguration},
    kms::key_management_service_server::KeyManagementServiceServer,
    utilities::{logging, socket::create_unix_socket},
    vault,
};
use tokio::join;
use tonic::transport::Server;

mod checks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::initialize();
    let socket_config = SocketConfiguration::new();
    let socket = create_unix_socket(&socket_config.socket_path, socket_config.permissions)?;
    let vault_config = VaultConfiguration::new();
    let vault_kms_server = vault::VaultKmsServer::new(
        &vault_config.vault_transit_key,
        &vault_config.vault_address,
        &vault_config.vault_role,
    );
    vault_kms_server.initialize().await?;
    let (server, health_checks) = join!(
        Server::builder()
            .add_service(KeyManagementServiceServer::new(vault_kms_server))
            .serve_with_incoming(socket),
        checks::serve()
    );
    server?;
    health_checks?;
    Ok(())
}
