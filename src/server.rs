extern crate lib;

use lib::{
    configuration::{
        authentication::Credentials, socket::SocketConfiguration, tls, vault::VaultConfiguration,
    },
    kms::key_management_service_server::KeyManagementServiceServer,
    utilities::{logging, socket::Socket, watcher},
    vault,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::Server;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

mod checks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::initialize();
    let socket_config = SocketConfiguration::new();
    let socket = Socket::with_permissions(&socket_config.permissions);
    let stream = socket.listen(&socket_config.socket_path)?;
    let vault_config = VaultConfiguration::new();
    let tls_config = tls::TlsConfiguration::new();
    let settings = VaultClientSettingsBuilder::default()
        .address(&vault_config.address)
        .identity(tls_config.identity())
        .ca_certs(tls_config.certs())
        .build()?;
    let client = Arc::new(RwLock::new(vault::Client::new(
        VaultClient::new(settings).unwrap(),
        &vault_config,
    )));
    let vault_kms_server = vault::VaultKmsServer::new(client.clone());
    vault_kms_server.initialize().await?;
    tokio::try_join!(
        async {
            Server::builder()
                .add_service(KeyManagementServiceServer::new(vault_kms_server))
                .serve_with_incoming(stream)
                .await
                .map_err(|error| std::io::Error::other(error.to_string()))
        },
        async {
            checks::serve()
                .await
                .map_err(|error| std::io::Error::other(error.to_string()))
        },
        watcher::watch(
            match vault_config.credentials {
                Credentials::Kubernetes(credentials) => credentials.jwt.path(),
                Credentials::AppRole(role) => role.secret_id.path(),
                Credentials::Token(token) => token.path(),
                Credentials::UserPass(credentials) => credentials.password.path(),
                _ => None,
            },
            client
        ),
    )?;
    Ok(())
}
