use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct EncryptionResponse {
  pub ciphertext: String
}

#[derive(Serialize)]
pub struct EncryptionRequest {
  plaintext: String,
}

impl EncryptionRequest {
  pub fn new(data: &Vec<u8>) -> Self {
    EncryptionRequest {
      plaintext: BASE64_STANDARD.encode(data),
    }
  }
}