use std::collections::HashMap;
use serde::Deserialize;

pub struct KeyInfo {
  pub id: String,
  pub version: String,
}

#[derive(Deserialize)]
pub struct KeyResponse {
  #[serde(rename = "type")]
  key_type: String,
  deletion_allowed: bool,
  derived: bool,
  exportable: bool,
  allow_plaintext_backup: bool,
  keys: HashMap<String, u32>,
  min_decryption_version: u32,
  min_encryption_version: u32,
  name: String,
  supports_encryption: bool,
  supports_decryption: bool,
  supports_derivation: bool,
  supports_signing: bool,
  imported: bool
}

impl KeyResponse {
  pub fn info(&self) -> KeyInfo {
    let mut keys: Vec<(String, String)> = self.keys.iter()
      .map(|(a, b)| (a.to_string(), b.to_string()))
      .collect::<Vec<(String, String)>>();
    keys.sort_by(| (a, _), (b, _) | a.cmp(b));
    let (version, id) = keys.first().unwrap();
    KeyInfo {
      version: version.to_string(),
      id: id.to_string(),
    }
  }
}

