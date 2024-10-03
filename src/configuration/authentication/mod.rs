mod app_role;
mod kubernetes;
mod user_pass;

pub use app_role::AppRole;
pub use kubernetes::Kubernetes;
pub use user_pass::UserPass;

use crate::utilities::{environment::Environment, source::Source};

const DEFAULT_USER: &str = "vault-kms-provider";

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
        let auth_mount = Environment::VaultAuthMount.get();
        if let Some(token) = Environment::VaultToken.source() {
            Self::Token(token)
        } else if let Some(jwt) = Environment::VaultKubernetesJwt.source() {
            Self::Kubernetes(Kubernetes::new(jwt, auth_mount))
        } else if let Some(password) = Environment::VaultPassword.source() {
            Self::UserPass(UserPass::new(
                Environment::VaultUser.or(DEFAULT_USER),
                password,
                auth_mount,
            ))
        } else if let Some((role_id, secret_id)) = Environment::VaultRoleId
            .get()
            .zip(Environment::VaultSecretId.source())
        {
            Self::AppRole(AppRole::new(role_id, secret_id, auth_mount))
        } else {
            Self::None
        }
    }
}
