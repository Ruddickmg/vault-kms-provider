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
  pub fn new<T: ToString>(data: &T) -> Self {
    DecryptionRequest {
      ciphertext: data.to_string()
    }
  }
}