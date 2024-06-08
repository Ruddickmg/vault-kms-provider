use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct EncryptionResponse {
  pub ciphertext: String
}

impl EncryptionResponse {
  pub fn from_json<'a, T: Deserialize<'a>>(data: T) -> Self {
    let something: EncryptionResponse = data.into();
    let text = something.ciphertext;
    EncryptionResponse {
      ciphertext: BASE64_STANDARD.decode(text.as_bytes())
        .expect(&format!("Invalid base 64 string: {}", &text))
        .into()
    }
  }
}

#[derive(Serialize)]
pub struct EncryptionRequest {
  plaintext: String,
}

impl EncryptionRequest {
  pub fn new(data: &[u8]) -> Self {
    EncryptionRequest {
      plaintext: BASE64_STANDARD.encode(data),
    }
  }
}