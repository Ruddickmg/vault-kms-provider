use crate::kms::api::{
    key_management_service_server::{KeyManagementService, KeyManagementServiceServer},
    DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse, StatusRequest,
    StatusResponse,
};
use crate::vault::keys::KeyInfo;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::string::ToString;
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status};
use vaultrs::{client, error::ClientError, transit};

const OKAY_RESPONSE: &str = "ok";
const TRANSIT_MOUNT: &str = "transit";

struct VaultKmsServer {
    client: client::VaultClient,
    key_name: String,
}

impl VaultKmsServer {
    pub fn new(name: &str, address: &str, token: &str) -> Self {
        let vault_settings = client::VaultClientSettingsBuilder::default()
            .address(address)
            .token(token)
            .build()
            .unwrap();
        VaultKmsServer {
            client: client::VaultClient::new(vault_settings).unwrap(),
            key_name: name.to_string(),
        }
    }
    async fn request_key(&self) -> Result<KeyInfo, ClientError> {
        Ok(
            transit::key::read(&self.client, TRANSIT_MOUNT, &self.key_name)
                .await?
                .keys
                .into(),
        )
    }
    async fn request_encryption(&self, data: &Vec<u8>) -> Result<String, ClientError> {
        let encoded = BASE64_STANDARD.encode(data);
        Ok(
            transit::data::encrypt(&self.client, TRANSIT_MOUNT, &self.key_name, &encoded, None)
                .await?
                .ciphertext,
        )
    }
    async fn request_decryption(&self, data: &Vec<u8>) -> Result<String, ClientError> {
        let encoded = BASE64_STANDARD.encode(data);
        Ok(
            transit::data::decrypt(&self.client, TRANSIT_MOUNT, &self.key_name, &encoded, None)
                .await?
                .plaintext,
        )
    }
}

#[tonic::async_trait]
impl KeyManagementService for VaultKmsServer {
    async fn status(&self, _: Request<StatusRequest>) -> Result<Response<StatusResponse>, Status> {
        self.request_key().await.map_or_else(
            |error| Err(Status::new(Code::Internal, error.to_string())),
            |key| {
                Ok(Response::new(StatusResponse {
                    version: key.version,
                    key_id: key.id,
                    healthz: OKAY_RESPONSE.to_string(),
                }))
            },
        )
    }

    async fn decrypt(
        &self,
        request: Request<DecryptRequest>,
    ) -> Result<Response<DecryptResponse>, Status> {
        self.request_decryption(&request.get_ref().ciphertext.to_vec())
            .await
            .map_or_else(
                |error| Err(Status::new(Code::Internal, error.to_string())),
                |plaintext| {
                    Ok(Response::new(DecryptResponse {
                        plaintext: plaintext.as_bytes().to_vec(),
                    }))
                },
            )
    }

    async fn encrypt(
        &self,
        request: Request<EncryptRequest>,
    ) -> Result<Response<EncryptResponse>, Status> {
        match self.request_encryption(&request.get_ref().plaintext).await {
            Ok(ciphertext) => self.request_key().await.map_or_else(
                |error| Err(Status::new(Code::Internal, error.to_string())),
                |key| {
                    Ok(Response::new(EncryptResponse {
                        key_id: key.id,
                        ciphertext: ciphertext.as_bytes().to_vec(),
                        annotations: HashMap::new(),
                    }))
                },
            ),
            Err(error) => Err(Status::new(Code::Internal, error.to_string())),
        }
    }
}

pub async fn server(address: SocketAddr) -> Result<(), tonic::transport::Error> {
    let vault_kms_server =
        VaultKmsServer::new("kms", "https://vault.vault.svc.local:8200", "token");
    println!("Server listening at socket: {}", address);
    Server::builder()
        .add_service(KeyManagementServiceServer::new(vault_kms_server))
        .serve(address)
        .await
}
