use crate::utilities::env::{get_env, get_env_option};

const DEFAULT_USER: &str = "vault-kms-provider";

#[derive(Clone, Debug)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}


#[derive(Clone, Debug)]
pub enum Auth {
  Kubernetes(String),
  Credentials(Credentials),
  Token(String),
  None,
}

impl Auth {
  pub fn get() -> Self {
    if let Some(token) = get_env_option("VAULT_TOKEN") {
      Self::Token(token)
    } else if let Some(path) = get_env_option("VAULT_JWT_PATH") {
      Self::Kubernetes(path)
    } else if let Some(credentials) = get_env_option("VAULT_PASSWORD").map(|password| Credentials {
      username: get_env("VAULT_USER", DEFAULT_USER),
      password,
    }) {
      Self::Credentials(credentials)
    } else {
      Self::None
    }
  }
}