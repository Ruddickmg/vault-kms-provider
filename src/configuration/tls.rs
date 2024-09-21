use std::fs;
use crate::utilities::env;

const VAULT_CA_PATH: &str = "VAULT_CA_PATH";
const VAULT_CA_CERTIFICATE: &str = "VAULT_CA_CERT";

pub struct TlsConfiguration {
  directory: Option<String>,
  file: Option<String>,
}

impl TlsConfiguration {
  pub fn new() -> Self {
    Self {
      directory: env::get_env_option(VAULT_CA_PATH),
      file: env::get_env_option(VAULT_CA_CERTIFICATE),
    }
  }

  pub fn certs(&self) -> Vec<String> {
    let mut certs = self.certs_from_dir();
    if let Some(file_path) = self.file.clone() {
      if !certs.contains(&file_path) {
        certs.push(file_path);
      }
    }
    certs
  }

  fn certs_from_dir(&self) -> Vec<String> {
    if let Some(path) = self.directory.clone() {
      if let Ok(paths) = fs::read_dir(&path) {
        let a: Vec<String> = paths
          .filter(| p | p.is_ok())
          .map(| p | p.unwrap().path())
          .filter(| p | p.is_file())
          .map(| p | p.to_str().unwrap().to_string())
          .filter(| p | p.as_str() != "")
          .collect();
        return a
      }
    }
    vec![]
  }
}