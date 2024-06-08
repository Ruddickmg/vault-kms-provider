use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct DecryptionResponse {
  pub plaintext: String
}

#[derive(Serialize)]
pub struct DecryptionRequest {
  ciphertext: String
}

impl DecryptionRequest {
  pub fn new(data: &Vec<u8>) -> Self {
    DecryptionRequest {
      ciphertext: BASE64_STANDARD.encode(data)
    }
  }
}