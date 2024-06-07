use std::fmt::Display;
use std::string::ToString;
use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};
use crate::api_grpc::KeyManagementService;
use crate::vault::api::{DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse, StatusRequest, StatusResponse};
use serde::Serialize;
use crate::vault::decryption::{DecryptionRequest, DecryptionResponse};
use crate::vault::encryption::{EncryptionRequest, EncryptionResponse};
use crate::vault::transit::TransitPath;
use reqwest;
use crate::vault::data::Data;
use crate::vault::keys::{KeyInfo, KeyResponse};

const VERSION: &str = "v2";
const OKAY_RESPONSE: &str = "ok";

type HttpResponse = reqwest::blocking::Response;

pub struct VaultKms {
  client: reqwest::blocking::Client,
  url: String,
  token: String,
  key_name: String,
}

impl VaultKms {
  pub fn new(name: &str, url: &str, token: &str) -> Self {
    VaultKms {
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

impl KeyManagementService for VaultKms {
  fn status(&self, _context: ServerHandlerContext, _req: ServerRequestSingle<StatusRequest>, resp: ServerResponseUnarySink<StatusResponse>) -> grpc::Result<()> {
    let key = self.request_key()?;
    resp.finish(StatusResponse {
      version: key.version,
      key_id: key.id,
      healthz: OKAY_RESPONSE.to_string(),
    })
  }

  fn decrypt(&self, _context: ServerHandlerContext, req: ServerRequestSingle<DecryptRequest>, resp: ServerResponseUnarySink<DecryptResponse>) -> grpc::Result<()> {
    let DecryptionResponse { plaintext }: DecryptionResponse = self.request_decryption(&req.message.ciphertext)?;
    resp.finish(DecryptResponse {
      plaintext: plaintext.as_bytes(),
    })
  }

  fn encrypt(&self, _context: ServerHandlerContext, req: ServerRequestSingle<EncryptRequest>, resp: ServerResponseUnarySink<EncryptResponse>) -> grpc::Result<()> {
    let EncryptionResponse { ciphertext }: EncryptionResponse = self.request_encryption(&req.message.plaintext)?;
    let key = self.request_key()?;
    resp.finish(EncryptResponse {
      key_id: key.id,
      ciphertext: ciphertext.as_bytes(),
      annotations: None,
    })
  }
}



