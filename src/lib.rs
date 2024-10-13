extern crate core;

use crate::configuration::ServerConfiguration;
use crate::kms::key_management_service_server::KeyManagementServiceServer;
use crate::utilities::{socket::Socket, watcher};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::Server;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

pub mod checks;
pub mod configuration;
pub mod utilities;
pub mod vault;
pub mod kms {
    tonic::include_proto!("v2");
}

pub async fn server(
    ServerConfiguration {
        socket: socket_config,
        tls: tls_config,
        vault: vault_config,
        health: health_config,
    }: ServerConfiguration,
) -> Result<(), Box<dyn std::error::Error>> {
    let socket = Socket::with_permissions(&socket_config.permissions);
    let stream = socket.listen(&socket_config.socket_path)?;
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
        checks::serve(&health_config.endpoint),
        watcher::watch_credentials(vault_config.credentials, client)
    )?;
    Ok(())
}
