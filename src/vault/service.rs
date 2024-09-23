use crate::configuration::vault::VaultConfiguration;
use crate::kms::{
    key_management_service_server::KeyManagementService, DecryptRequest, DecryptResponse,
    EncryptRequest, EncryptResponse, StatusRequest, StatusResponse,
};
use crate::vault::keys::KeyInfo;
use base64::{prelude::BASE64_STANDARD, Engine};
use std::{collections::HashMap, fs, io::ErrorKind, string::ToString, sync::Arc, sync::atomic::{AtomicBool, Ordering}};
use tonic::{Code, Request, Response, Status};
use tracing::{debug, info, instrument, warn};
use vaultrs::{api::AuthInfo, error::ClientError, transit, client::{Client, VaultClient}};

const OKAY_RESPONSE: &str = "ok";
const TRANSIT_MOUNT: &str = "transit";
const KUBERNETES_AUTH_MOUNT: &str = "kubernetes";

#[derive(Debug)]
struct VaultError(ClientError);

impl From<VaultError> for Status {
    fn from(value: VaultError) -> Self {
        Status::new(Code::Internal, value.0.to_string())
    }
}

impl From<ClientError> for VaultError {
    fn from(value: ClientError) -> Self {
        VaultError(value)
    }
}

pub struct VaultKmsServer {
    role: String,
    key_name: String,
    token: Option<String>,
    jwt_path: Option<String>,
    client: VaultClient,
    rotate: Arc<AtomicBool>
}

impl VaultKmsServer {

    #[instrument(skip(self, path))]
    async fn request_token_with_jwt(&self, path: &str) -> Result<String, ClientError> {
        let jwt = fs::read_to_string(path).map_err(|_| ClientError::FileNotFoundError {
            path: path.to_string(),
        })?;
        debug!("Using mounted jwt of length: {}", jwt.len());
        Ok(self.jwt_auth(&jwt).await?.client_token)
    }

    #[instrument(skip(self))]
    async fn get_token(&self) -> Result<Option<String>, ClientError> {
        let token = if self.rotate.load(Ordering::Relaxed) {
            if let Some(path) = self.jwt_path.clone() {
                Some(self.request_token_with_jwt(&path).await?)
            } else if let Some(token) = self.token.clone() {
                Some(token)
            } else {
                warn!("No token found");
                None
            }
        } else {
            None
        };
        self.rotate.swap(false, Ordering::Relaxed);
        Ok(token)
    }

    #[instrument(skip(self))]
   async fn refresh_token(&mut self) -> Result<(), ClientError> {
        if let Some(token) = self.get_token().await? {
            self.client.set_token(&token);
        }
        Ok(())
    }

    pub fn new(client: VaultClient, config: &VaultConfiguration, rotate: Arc<AtomicBool>) -> Self {
        Self {
            role: config.vault_role.to_string(),
            key_name: config.vault_transit_key.to_string(),
            jwt_path: config.jwt_path.clone(),
            token: config.vault_token.clone(),
            rotate,
            client,
        }
    }

    #[instrument(skip(self))]
    pub async fn initialize(&mut self) -> Result<(), std::io::Error> {
        self.refresh_token().await.map_err(|e| std::io::Error::other(e.to_string()))?;
        self.request_encryption(&BASE64_STANDARD.encode("initialize".as_bytes()))
            .await
            .map_err(|error| {
                let error = format!("Failed to initialize: {}", error.0.to_string());
                std::io::Error::new(ErrorKind::Other, error.as_str())
            })?;
        info!(
            "Encryption key: \"{}\" has been initialized in vault",
            self.key_name
        );
        Ok(())
    }

    #[instrument(skip(self, jwt))]
    async fn jwt_auth(&self, jwt: &str) -> Result<AuthInfo, ClientError> {
        debug!("Logging in to vault as: {}", self.role);
        Ok(vaultrs::auth::kubernetes::login(&self.client, KUBERNETES_AUTH_MOUNT, &self.role, &jwt).await?)
    }

    #[instrument(skip(self))]
    async fn request_key(&self) -> Result<KeyInfo, VaultError> {
        Ok(
            transit::key::read(&self.client, TRANSIT_MOUNT, &self.key_name)
                .await?
                .keys
                .into(),
        )
    }

    #[instrument(skip(self, data))]
    async fn request_encryption(&self, data: &str) -> Result<String, VaultError> {
        debug!("Requesting encryption, data: {}", data);
        Ok(
            transit::data::encrypt(&self.client, TRANSIT_MOUNT, &self.key_name, data, None)
                .await?
                .ciphertext,
        )
    }

    #[instrument(skip(self, data))]
    async fn request_decryption(&self, data: &str) -> Result<String, VaultError> {
        debug!("Requesting decryption, data: {}", data);
        Ok(
            transit::data::decrypt(&self.client, TRANSIT_MOUNT, &self.key_name, data, None)
                .await?
                .plaintext,
        )
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
        let key = self.request_key().await?;
        Ok(Response::new(StatusResponse {
            version: key.version,
            key_id: key.id,
            healthz: OKAY_RESPONSE.to_string(),
        }))
    }

    #[instrument(skip(self, request))]
    async fn decrypt(
        &self,
        request: Request<DecryptRequest>,
    ) -> Result<Response<DecryptResponse>, Status> {
        info!("Decryption request");
        let encrypted = String::from_utf8(request.get_ref().ciphertext.to_vec())
            .map_err(|error| Status::new(Code::Internal, error.to_string()))?;
        let plaintext = self.request_decryption(&encrypted).await?;
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
        let encoded = BASE64_STANDARD.encode(&request.get_ref().plaintext);
        let ciphertext = self.request_encryption(&encoded).await?;
        let key = self.request_key().await?;
        let response = Ok(Response::new(EncryptResponse {
            key_id: key.id,
            ciphertext: ciphertext.as_bytes().to_vec(),
            annotations: HashMap::new(),
        }));
        info!("Encryption successful");
        response
    }
}
