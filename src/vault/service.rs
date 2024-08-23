use crate::kms::{
    key_management_service_server::KeyManagementService, DecryptRequest, DecryptResponse,
    EncryptRequest, EncryptResponse, StatusRequest, StatusResponse,
};
use crate::utilities::token;
use crate::vault::keys::KeyInfo;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::string::ToString;
use tonic::{Code, Request, Response, Status};
use tracing::{debug, info, instrument};
use vaultrs::{client, error::ClientError, transit};

const OKAY_RESPONSE: &str = "ok";
const TRANSIT_MOUNT: &str = "transit";

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

#[derive(Debug)]
pub struct VaultKmsServer {
    address: String,
    key_name: String,
}

impl VaultKmsServer {
    #[instrument(skip(self))]
    fn get_client(&self) -> client::VaultClient {
        let token = token::auth_token().unwrap();
        let vault_settings = client::VaultClientSettingsBuilder::default()
            .address(&self.address)
            .token(token)
            .build()
            .unwrap();
        client::VaultClient::new(vault_settings).unwrap()
    }

    #[instrument]
    pub fn new(name: &str, address: &str) -> Self {
        VaultKmsServer {
            address: address.to_string(),
            key_name: name.to_string(),
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

    #[instrument(skip(self))]
    async fn request_key(&self) -> Result<KeyInfo, VaultError> {
        Ok(
            transit::key::read(&self.get_client(), TRANSIT_MOUNT, &self.key_name)
                .await?
                .keys
                .into(),
        )
    }

    #[instrument(skip(self, data))]
    async fn request_encryption(&self, data: &str) -> Result<String, VaultError> {
        debug!("Requesting encryption, data: {}", data);
        Ok(transit::data::encrypt(
            &self.get_client(),
            TRANSIT_MOUNT,
            &self.key_name,
            data,
            None,
        )
        .await?
        .ciphertext)
    }

    #[instrument(skip(self, data))]
    async fn request_decryption(&self, data: &str) -> Result<String, VaultError> {
        debug!("Requesting decryption, data: {}", data);
        Ok(transit::data::decrypt(
            &self.get_client(),
            TRANSIT_MOUNT,
            &self.key_name,
            data,
            None,
        )
        .await?
        .plaintext)
    }
}

#[tonic::async_trait]
impl KeyManagementService for VaultKmsServer {
    #[instrument]
    async fn status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        debug!("Status request: {:#?}", request);
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
        debug!("Decryption request: {:?}", request);
        let encrypted = String::from_utf8(request.get_ref().ciphertext.to_vec())
            .map_err(|error| Status::new(Code::Internal, error.to_string()))?;
        let plaintext = self.request_decryption(&encrypted).await?;
        Ok(Response::new(DecryptResponse {
            plaintext: BASE64_STANDARD
                .decode(plaintext.as_bytes())
                .map_err(|error| Status::new(Code::Internal, error.to_string()))?,
        }))
    }

    #[instrument(skip(self, request))]
    async fn encrypt(
        &self,
        request: Request<EncryptRequest>,
    ) -> Result<Response<EncryptResponse>, Status> {
        debug!("Encryption request: {:?}", request);
        let encoded = BASE64_STANDARD.encode(&request.get_ref().plaintext);
        let ciphertext = self.request_encryption(&encoded).await?;
        let key = self.request_key().await?;
        Ok(Response::new(EncryptResponse {
            key_id: key.id,
            ciphertext: ciphertext.as_bytes().to_vec(),
            annotations: HashMap::new(),
        }))
    }
}
