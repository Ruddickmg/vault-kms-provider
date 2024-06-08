use std::collections::HashMap;
use std::net::SocketAddr;
use std::string::ToString;
use serde::Serialize;
use crate::vault::transit::TransitPath;
use reqwest;
use crate::vault::keys::{KeyInfo, KeyResponse};
use tonic::{Code, Request, Response, Status};
use tonic::transport::Server;
use crate::kms::api;
use crate::kms::api::{DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse, StatusRequest, StatusResponse};
use crate::kms::api::key_management_service_server::KeyManagementServiceServer;
use crate::vault::data::Data;
use crate::vault::decryption::{DecryptionRequest, DecryptionResponse};
use crate::vault::encryption::{EncryptionRequest, EncryptionResponse};
use crate::vault::service;

const OKAY_RESPONSE: &str = "ok";

type HttpResponse = reqwest::Response;

pub struct VaultKmsServer {
  client: reqwest::Client,
  url: String,
  token: String,
  key_name: String,
}

impl VaultKmsServer {
  pub fn new(name: &str, url: &str, token: &str) -> Self {
    VaultKmsServer {
      client: reqwest::Client::new(),
      key_name: name.to_string(),
      url: url.to_string(),
      token: token.to_string(),
    }
  }
  fn get_url(&self, path: TransitPath) -> String {
    format!("{}/transit/{}/{}", &self.url, path, &self.key_name)
  }
  async fn request<T: Serialize>(&self, path: TransitPath, data: &T) -> Result<HttpResponse, reqwest::Error> {
    let url = self.get_url(path);
    self.client.post(&url).json(data).send().await
  }
  async fn request_key(&self) -> Result<KeyInfo, reqwest::Error> {
    let url = self.get_url(TransitPath::Keys);
    let Data { data }: Data<KeyResponse> = self.client.get(url).send().await?.json().await?;
    Ok(data.info())
  }
  async fn request_encryption(&self, data: &Vec<u8>) -> Result<EncryptionResponse, reqwest::Error> {
    let Data { data } : Data<EncryptionResponse> = self.request(TransitPath::Encrypt, &EncryptionRequest::new(data)).await?.json().await?;
    Ok(data)
  }
  async fn request_decryption(&self, data: &Vec<u8>) -> Result<DecryptionResponse, reqwest::Error> {
    let Data { data }: Data<DecryptionResponse> = self.request(TransitPath::Decrypt, &DecryptionRequest::new(data)).await?.json().await?;
    Ok(data)
  }
}

#[tonic::async_trait]
impl api::key_management_service_server::KeyManagementService for VaultKmsServer {
  async fn status(&self, _: Request<StatusRequest>) -> Result<Response<StatusResponse>, Status> {
    self.request_key().await.map_or_else(
      | error | Err(Status::new(Code::Internal, error.to_string())),
      | key | Ok(Response::new(StatusResponse {
      version: key.version,
      key_id: key.id,
      healthz: OKAY_RESPONSE.to_string(),
    })))
  }

  async fn decrypt(&self, request: Request<DecryptRequest>) -> Result<Response<DecryptResponse>, Status> {
    self.request_decryption(&request.get_ref().ciphertext.to_vec()).await.map_or_else(
      | error | Err(Status::new(Code::Internal, error.to_string())),
      | response | Ok(Response::new(DecryptResponse {
        plaintext: response.plaintext.as_bytes().to_vec(),
      })))
  }

  async fn encrypt(&self, request: Request<EncryptRequest>) -> Result<Response<EncryptResponse>, Status> {
    if let Ok(response) = self.request_encryption(&request.get_ref().plaintext).await {
      self.request_key().await.map_or_else(
        |_| Err(Status::new(Code::Internal, "Failed to retrieve key data")),
        | key | Ok(Response::new(EncryptResponse {
          key_id: key.id,
          ciphertext: response.ciphertext.as_bytes().to_vec(),
          annotations: HashMap::new(),
        })))
    } else {
      Err(Status::new(Code::Internal, "Failed encryption"))
    }
  }
}

pub async fn server(address: SocketAddr) -> Result<(), tonic::transport::Error> {
  let vault_kms_server = service::VaultKmsServer::new("kms", "https://vault.vault.svc.local:8200", "token");
  println!("Server listening at socket: {}", address);
  Server::builder()
    .add_service(KeyManagementServiceServer::new(vault_kms_server))
    .serve(address)
    .await
}
