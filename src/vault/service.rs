use crate::kms::{
    key_management_service_server::KeyManagementService, DecryptRequest, DecryptResponse,
    EncryptRequest, EncryptResponse, StatusRequest, StatusResponse,
};
use crate::utilities::watcher::Refresh;
use crate::vault::client;
use base64::{prelude::BASE64_STANDARD, Engine};
use std::sync::Arc;
use std::{collections::HashMap, io::ErrorKind, string::ToString};
use tokio::sync::RwLock;
use tonic::{Code, Request, Response, Status};
use tracing::{debug, error, info, instrument};

const OKAY_RESPONSE: &str = "ok";

pub struct VaultKmsServer {
    client: Arc<RwLock<client::Client>>,
}

impl VaultKmsServer {
    pub fn new(client: Arc<RwLock<client::Client>>) -> Self {
        Self { client }
    }

    #[instrument(skip(self))]
    pub async fn initialize(&self) -> Result<(), std::io::Error> {
        let mut client = self.client.write().await;
        client.refresh_token().await.map_err(|e| {
            error!("Failed to authenticate during initialization: {:?}", e);
            std::io::Error::other(e.to_string())
        })?;
        client
            .request_encryption(&BASE64_STANDARD.encode("initialize".as_bytes()))
            .await
            .map_err(|error| {
                let error = format!("Failed to initialize: {}", error.0.to_string());
                error!("{}", error);
                std::io::Error::new(ErrorKind::Other, error.as_str())
            })?;
        info!("Vault encryption has been initialized");
        Ok(())
    }
}

#[tonic::async_trait]
impl KeyManagementService for VaultKmsServer {
    #[instrument(skip(self, _request))]
    async fn status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        debug!("Status request");
        let client = self.client.read().await;
        Ok(client.request_key().await.map(|key| {
            Response::new(StatusResponse {
                version: key.version,
                key_id: key.id,
                healthz: OKAY_RESPONSE.to_string(),
            })
        })?)
    }

    #[instrument(skip(self, request))]
    async fn decrypt(
        &self,
        request: Request<DecryptRequest>,
    ) -> Result<Response<DecryptResponse>, Status> {
        info!("Decryption request");
        let client = self.client.read().await;
        let encrypted = String::from_utf8(request.get_ref().ciphertext.to_vec())
            .map_err(|error| Status::new(Code::Internal, error.to_string()))?;
        let plaintext = client.request_decryption(&encrypted).await?;
        let response = Ok(Response::new(DecryptResponse {
            plaintext: BASE64_STANDARD
                .decode(plaintext.as_bytes())
                .map_err(|error| Status::new(Code::Internal, error.to_string()))?,
        }));
        info!("Decryption successful");
        response
    }

    #[instrument(skip(self, request))]
    async fn encrypt(
        &self,
        request: Request<EncryptRequest>,
    ) -> Result<Response<EncryptResponse>, Status> {
        info!("Encryption request");
        let client = self.client.read().await;
        let encoded = BASE64_STANDARD.encode(&request.get_ref().plaintext);
        let ciphertext = client.request_encryption(&encoded).await?;
        let key = client.request_key().await?;
        let response = Ok(Response::new(EncryptResponse {
            key_id: key.id,
            ciphertext: ciphertext.as_bytes().to_vec(),
            annotations: HashMap::new(),
        }));
        info!("Encryption successful");
        response
    }
}
