extern crate lib;

use lib::{
    configuration::{socket::SocketConfiguration, tls, vault::VaultConfiguration},
    kms::key_management_service_server::KeyManagementServiceServer,
    utilities::{logging, socket::create_unix_socket, watcher},
    vault,
};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::join;
use tonic::transport::Server;

mod checks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::initialize();
    let rotate_token: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    let socket_config = SocketConfiguration::new();
    let socket = create_unix_socket(&socket_config.socket_path, socket_config.permissions)?;
    let vault_config = VaultConfiguration::new();
    let tls_config = tls::TlsConfiguration::new();
    let vault_kms_server = vault::VaultKmsServer::new(&vault_config, tls_config.certs(), rotate_token.clone());
    vault_kms_server.initialize().await?;
    let (server, health_checks, watch) = join!(
        Server::builder()
            .add_service(KeyManagementServiceServer::new(vault_kms_server))
            .serve_with_incoming(socket),
        checks::serve(),
        watcher::watch("test_files/hello.ts")
    );
    server?;
    health_checks?;
    watch?;
    Ok(())
}
