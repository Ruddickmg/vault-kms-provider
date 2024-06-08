use std::collections::HashMap;
use std::fmt::Display;
use std::string::ToString;
use serde::Serialize;
use crate::vault::transit::TransitPath;
use reqwest;
use crate::vault::data::Data;
use crate::vault::keys::{KeyInfo, KeyResponse};
use tonic::{Request, Response, Status};
use crate::kms::api;
use crate::kms::api::{DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse, StatusRequest, StatusResponse};
use crate::vault::decryption::{DecryptionRequest, DecryptionResponse};
use crate::vault::encryption::{EncryptionRequest, EncryptionResponse};

const VERSION: &str = "v2";
const OKAY_RESPONSE: &str = "ok";

type HttpResponse = reqwest::blocking::Response;

pub struct VaultKmsServer {
  client: reqwest::blocking::Client,
  url: String,
  token: String,
  key_name: String,
}

impl VaultKmsServer {
  pub fn new(name: &str, url: &str, token: &str) -> Self {
    VaultKmsServer {
      client: reqwest::blocking::Client::new(),
      key_name: name.to_string(),
      url: url.to_string(),
      token: token.to_string(),
    }
  }
  fn get_url(&self, path: TransitPath) -> String {
    format!("{}/transit/{}/{}", &self.url, path, &self.key_name)
  }
  fn request<T: Serialize>(&self, path: TransitPath, data: &T) -> HttpResponse {
    let url = self.get_url(path);
    let resp = self.client.post(&url).json(data).send()?;
    println!("decryption response: {:#?}", resp);
    resp
  }
  fn request_key(&self) -> Result<KeyInfo, Self::Error> {
    let url = self.get_url(TransitPath::Keys);
    let Data { data }: Data<KeyResponse> = self.client.get(url).send()?.json()?;
    Ok(data.info())
  }
  fn request_encryption(&self, data: &[u8]) -> Result<EncryptionResponse, Self::Error> {
    let Data { data }: Data<EncryptionResponse> = self.request(TransitPath::Encrypt, &EncryptionRequest::new(data)).json()?;
    Ok(data)
  }
  fn request_decryption<T: Display>(&self, data: &T) -> Result<DecryptionResponse, Self::Error> {
    let Data{ data }: Data<DecryptionResponse> = self.request(TransitPath::Decrypt, &DecryptionRequest::new(data)).json()?;
    Ok(data)
  }
}

#[tonic::async_trait]
impl api::key_management_service_server::KeyManagementService for VaultKmsServer {
  async fn status(&self, _: Request<StatusRequest>) -> Result<Response<StatusResponse>, Status> {
    let key = self.request_key()?;
    Ok(Response::new(StatusResponse {
      version: key.version,
      key_id: key.id,
      healthz: OKAY_RESPONSE.to_string(),
    }))
  }

  async fn decrypt(&self, request: Request<DecryptRequest>) -> Result<Response<DecryptResponse>, Status> {
    let DecryptionResponse { plaintext }: DecryptionResponse = self.request_decryption(&request.get_ref().ciphertext)?;
    Ok(Response::new(DecryptResponse {
      plaintext: plaintext.as_bytes(),
    }))
  }

  async fn encrypt(&self, request: Request<EncryptRequest>) -> Result<Response<EncryptResponse>, Status> {
    let EncryptionResponse { ciphertext }: EncryptionResponse = self.request_encryption(request.get_ref().plaintext)?;
    let key = self.request_key()?;
    Ok(Response::new(EncryptResponse {
      key_id: key.id,
      ciphertext: ciphertext.as_bytes(),
      annotations: HashMap::new(),
    }))
  }
}




