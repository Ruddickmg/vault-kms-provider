use crate::kms::{
    key_management_service_server::KeyManagementService, DecryptRequest, DecryptResponse,
    EncryptRequest, EncryptResponse, StatusRequest, StatusResponse,
};
use crate::utilities::token;
use crate::vault::keys::KeyInfo;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::collections::HashMap;
use std::string::ToString;
use tonic::{Code, Request, Response, Status};
use vaultrs::{client, error::ClientError, transit};

const OKAY_RESPONSE: &str = "ok";
const TRANSIT_MOUNT: &str = "transit";

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
    address: String,
    key_name: String,
}

impl VaultKmsServer {
    fn get_client(&self) -> client::VaultClient {
        let token = token::auth_token().unwrap();
        let vault_settings = client::VaultClientSettingsBuilder::default()
            .address(&self.address)
            .token(token)
            .build()
            .unwrap();
        client::VaultClient::new(vault_settings).unwrap()
    }

    pub fn new(name: &str, address: &str) -> Self {
        VaultKmsServer {
            address: address.to_string(),
            key_name: name.to_string(),
        }
    }
    async fn request_key(&self) -> Result<KeyInfo, VaultError> {
        Ok(
            transit::key::read(&self.get_client(), TRANSIT_MOUNT, &self.key_name)
                .await?
                .keys
                .into(),
        )
    }

    async fn request_encryption(&self, data: &str) -> Result<String, VaultError> {
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
    async fn request_decryption(&self, data: &str) -> Result<String, VaultError> {
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
    async fn status(&self, _: Request<StatusRequest>) -> Result<Response<StatusResponse>, Status> {
        println!("getting status");
        let key = self.request_key().await?;
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
        println!("decrypting");
        let encrypted = String::from_utf8(request.get_ref().ciphertext.to_vec())
            .map_err(|error| Status::new(Code::Internal, error.to_string()))?;
        let plaintext = self.request_decryption(&encrypted).await?;
        Ok(Response::new(DecryptResponse {
            plaintext: BASE64_STANDARD
                .decode(plaintext.as_bytes())
                .map_err(|error| Status::new(Code::Internal, error.to_string()))?,
        }))
    }

    async fn encrypt(
        &self,
        request: Request<EncryptRequest>,
    ) -> Result<Response<EncryptResponse>, Status> {
        println!("encrypting");
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
