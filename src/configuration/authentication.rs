use crate::utilities::env::{get_env, get_env_option};

const DEFAULT_USER: &str = "vault-kms-provider";
const DEFAULT_PATH: &str = "userpass";

#[derive(Clone, Debug)]
pub struct Credentials {
  pub username: String,
  pub password: String,
  pub path: String,
}


#[derive(Clone, Debug)]
pub enum Authentication {
  Kubernetes(String),
  Credentials(Credentials),
  Token(String),
  None,
}

impl Authentication {
  pub fn get() -> Self {
    if let Some(token) = get_env_option("VAULT_TOKEN") {
      Self::Token(token)
    } else if let Some(path) = get_env_option("VAULT_JWT_PATH") {
      Self::Kubernetes(path)
    } else if let Some(credentials) = get_env_option("VAULT_PASSWORD").map(|password| Credentials {
      username: get_env("VAULT_USER", DEFAULT_USER),
      path: get_env("VAULT_USER_LOGIN_PATH", DEFAULT_PATH),
      password,
    }) {
      Self::Credentials(credentials)
    } else {
      Self::None
    }
  }
}