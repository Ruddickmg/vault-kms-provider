use crate::kms::{
    key_management_service_server::KeyManagementService, DecryptRequest, DecryptResponse,
    EncryptRequest, EncryptResponse, StatusRequest, StatusResponse,
};
use crate::vault::client;
use base64::{prelude::BASE64_STANDARD, Engine};
use std::{collections::HashMap, fs, io::ErrorKind, string::ToString,sync::atomic::{Ordering}};
use tonic::{Code, Request, Response, Status};
use tracing::{debug, info, instrument, warn};
use vaultrs::client::Client;
use vaultrs::error::ClientError;
use tokio::sync::RwLock;
use std::borrow::Borrow;
use std::sync::{Arc};

const OKAY_RESPONSE: &str = "ok";

pub struct VaultKmsServer {
    client: Arc<RwLock<client::Client>>,
}

impl VaultKmsServer {

    #[instrument(skip(self, path))]
    async fn request_token_with_jwt(&self, path: &str) -> Result<String, ClientError> {
        let client = self.client.read().await;
        let jwt = fs::read_to_string(path).map_err(|_| ClientError::FileNotFoundError {
            path: path.to_string(),
        })?;
        debug!("Using mounted jwt of length: {}", jwt.len());
        Ok(client.jwt_auth(&jwt).await?.client_token)
    }

    #[instrument(skip(self))]
    async fn get_token(&self) -> Result<Option<String>, ClientError> {
        let client = self.client.read().await;
        let token = if client.rotate.load(Ordering::Relaxed) {
            if let Some(path) = client.jwt_path.clone() {
                Some(self.request_token_with_jwt(&path).await?)
            } else if let Some(token) = client.token.clone() {
                Some(token)
            } else {
                warn!("No token found");
                None
            }
        } else {
            None
        };
        client.borrow().rotate.swap(false, Ordering::Relaxed);
        Ok(token)
    }

    #[instrument(skip(self))]
   async fn refresh_token(&self) -> Result<(), ClientError> {
        let mut client = self.client.write().await;
        if let Some(token) = self.get_token().await? {
            client.client.set_token(&token);
        }
        Ok(())
    }

    pub fn new(client: client::Client) -> Self {
        Self { client: Arc::new(RwLock::new(client)) }
    }

    #[instrument(skip(self))]
    pub async fn initialize(&self) -> Result<(), std::io::Error> {
        let client = self.client.read().await;
        self.refresh_token().await.map_err(|e| std::io::Error::other(e.to_string()))?;
        client.request_encryption(&BASE64_STANDARD.encode("initialize".as_bytes()))
            .await
            .map_err(|error| {
                let error = format!("Failed to initialize: {}", error.0.to_string());
                std::io::Error::new(ErrorKind::Other, error.as_str())
            })?;
        info!(
            "Encryption key: \"{}\" has been initialized in vault",
            client.key_name
        );
        Ok(())
    }
}

#[tonic::async_trait]
impl KeyManagementService for VaultKmsServer {
    async fn status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        debug!("Status request");
        let client = self.client.read().await;
        let key = client.request_key().await?;
        Ok(Response::new(StatusResponse {
            version: key.version,
            key_id: key.id,
            healthz: OKAY_RESPONSE.to_string(),
        }))
    }

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
