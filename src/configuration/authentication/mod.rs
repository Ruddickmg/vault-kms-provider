mod app_role;
mod kubernetes;
mod user_pass;

use std::{fs, io};
pub use app_role::AppRole;
pub use kubernetes::Kubernetes;
pub use user_pass::UserPass;

use crate::utilities::env::{get_env, get_env_option, get_env_source_option};

const DEFAULT_USER: &str = "vault-kms-provider";

#[derive(Clone, Debug)]
pub enum Source {
    Value(String),
    FilePath(String),
}

impl Source {
    pub fn value(&self) -> io::Result<String> {
        match self {
            Self::Value(value) => Ok(value.to_string()),
            Self::FilePath(path) => fs::read_to_string(path)
        }
    }

    pub fn path(&self) -> Option<String> {
        if let Self::FilePath(path) = self {
            Some(path.to_string())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub enum Credentials {
    AppRole(AppRole),
    UserPass(UserPass),
    Kubernetes(Kubernetes),
    Token(Source),
    None,
}

impl Credentials {
    pub fn from_env() -> Self {
        let auth_mount = get_env_option("VAULT_AUTH_MOUNT");
        if let Some(token) = get_env_source_option("VAULT_TOKEN") {
            Self::Token(token)
        } else if let Some(jwt) = get_env_source_option("VAULT_KUBERNETES_JWT") {
            Self::Kubernetes(Kubernetes::new(jwt, auth_mount))
        } else if let Some(password) = get_env_source_option("VAULT_PASSWORD") {
            Self::UserPass(UserPass::new(
                get_env("VAULT_USER", DEFAULT_USER),
                password,
                auth_mount,
            ))
        } else if let Some((role_id, secret_id)) =
            get_env_option("VAULT_ROLE_ID").zip(get_env_source_option("VAULT_SECRET_ID"))
        {
            Self::AppRole(AppRole::new(role_id, secret_id, auth_mount))
        } else {
            Self::None
        }
    }
}
