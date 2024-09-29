mod kubernetes;
mod user_pass;

pub use kubernetes::Kubernetes;
pub use user_pass::UserPass;

use crate::utilities::env::{get_env, get_env_option};

const DEFAULT_USER: &str = "vault-kms-provider";

#[derive(Clone, Debug)]
pub enum Credentials {
  UserPass(UserPass),
  Kubernetes(Kubernetes),
  Token(String),
  None,
}

impl Credentials {
  pub fn from_env() -> Self {
    let auth_path = get_env_option("VAULT_AUTH_PATH");
    if let Some(token) = get_env_option("VAULT_TOKEN") {
      Self::Token(token)
    } else if let Some(file_path) = get_env_option("VAULT_JWT_PATH") {
      Self::Kubernetes(Kubernetes::new(file_path, auth_path))
    } else if let Some(password) = get_env_option("VAULT_PASSWORD") {
      Self::UserPass(UserPass::new(get_env("VAULT_USER", DEFAULT_USER), password, auth_path))
    } else {
      Self::None
    }
  }
}
