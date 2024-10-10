extern crate core;

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::configuration::socket::SocketConfiguration;
use crate::kms::{
    key_management_service_client::KeyManagementServiceClient,
    key_management_service_server::KeyManagementServiceServer,
};
use crate::utilities::socket::Socket;
use tonic::transport::{Channel, Server};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use crate::configuration::authentication::Credentials;
use crate::configuration::tls;
use crate::configuration::vault::VaultConfiguration;
use crate::utilities::watcher;

pub mod checks;
pub mod configuration;
pub mod utilities;
pub mod vault;
pub mod kms {
    tonic::include_proto!("v2");
}

pub async fn client() -> Result<KeyManagementServiceClient<Channel>, tonic::transport::Error> {
    let config = SocketConfiguration::new();
    let socket = Socket::default();
    let channel = socket.connect(&config.socket_path).await?;
    Ok(KeyManagementServiceClient::new(channel))
}

pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
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