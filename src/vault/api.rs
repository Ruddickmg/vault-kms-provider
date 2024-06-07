use std::iter::Map;

type Bytes<'a> = &'a[u8];
type Annotations = Option<Map<String, String>>;

pub struct StatusRequest {}

pub struct StatusResponse {
  pub version: String,
  pub healthz: String,
  pub key_id: String
}

pub struct DecryptRequest<'a> {
  pub key_id: String,
  pub uid: String,
  pub ciphertext: Bytes<'a>,
  pub annotations: Annotations,
}

pub struct DecryptResponse<'a> {
  pub plaintext: Bytes<'a>,
}

pub struct EncryptRequest<'a> {
  pub plaintext: Bytes<'a>,
  pub uid: String,
}

pub struct EncryptResponse<'a> {
  pub ciphertext: Bytes<'a>,
  pub key_id: String,
  pub annotations: Annotations,
}