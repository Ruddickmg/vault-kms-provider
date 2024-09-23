use crate::configuration::vault::VaultConfiguration;
use crate::kms::{
    key_management_service_server::KeyManagementService, DecryptRequest, DecryptResponse,
    EncryptRequest, EncryptResponse, StatusRequest, StatusResponse,
};
use crate::vault::keys::KeyInfo;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::string::ToString;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tonic::{Code, Request, Response, Status};
use tracing::{debug, info, instrument, warn};
use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
use vaultrs::{client, error::ClientError, transit};
use vaultrs::api::AuthInfo;

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
    address: String,
    certificates: Vec<String>,
    token: Option<String>,
    jwt_path: Option<String>,
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
        if token.is_some() {
            self.rotate.swap(false, Ordering::Relaxed);
        }
        Ok(token)
    }

    fn unauthenticated_client(&self) -> Result<VaultClient, ClientError> {
        let settings = client::VaultClientSettingsBuilder::default()
          .address(&self.address)
          .ca_certs(self.certificates.clone())
          .build()
          .unwrap();
        VaultClient::new(settings)
    }

   async fn get_client(&self) -> Result<VaultClient, ClientError> {
        let mut client = self.unauthenticated_client()?;
        if let Some(token) = self.get_token().await? {
            client.set_token(&token);
        }
        Ok(client)
    }

    pub fn new(config: &VaultConfiguration, certificates: Vec<String>, rotate: Arc<AtomicBool>) -> Self {
        Self {
            role: config.vault_role.to_string(),
            key_name: config.vault_transit_key.to_string(),
            address: config.vault_address.to_string(),
            jwt_path: config.vault_token_path.clone(),
            token: config.vault_token.clone(),
            certificates,
            rotate,
        }
    }

    #[instrument(skip(self))]
    pub async fn initialize(&self) -> Result<(), std::io::Error> {
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
        Ok(vaultrs::auth::kubernetes::login(&self.unauthenticated_client()?, KUBERNETES_AUTH_MOUNT, &self.role, &jwt).await?)
    }

    #[instrument(skip(self))]
    async fn request_key(&self) -> Result<KeyInfo, VaultError> {
        Ok(
            transit::key::read(&self.get_client().await?, TRANSIT_MOUNT, &self.key_name)
                .await?
                .keys
                .into(),
        )
    }

    #[instrument(skip(self, data))]
    async fn request_encryption(&self, data: &str) -> Result<String, VaultError> {
        debug!("Requesting encryption, data: {}", data);
        Ok(
            transit::data::encrypt(&self.get_client().await?, TRANSIT_MOUNT, &self.key_name, data, None)
                .await?
                .ciphertext,
        )
    }

    #[instrument(skip(self, data))]
    async fn request_decryption(&self, data: &str) -> Result<String, VaultError> {
        debug!("Requesting decryption, data: {}", data);
        Ok(
            transit::data::decrypt(&self.get_client().await?, TRANSIT_MOUNT, &self.key_name, data, None)
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
