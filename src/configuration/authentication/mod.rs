mod app_role;
mod certificate;
mod jwt;
mod kubernetes;
mod user_pass;

pub use app_role::AppRole;
pub use certificate::Certificate;
pub use jwt::Jwt;
pub use kubernetes::Kubernetes;
pub use user_pass::UserPass;

use crate::utilities::{environment::Environment, source::Source};

const DEFAULT_USER: &str = "vault-kms-provider";

#[derive(Clone, Debug)]
pub enum Credentials {
    AppRole(AppRole),
    UserPass(UserPass),
    Kubernetes(Kubernetes),
    Certificate(Certificate),
    Token(Source),
    Jwt(Jwt),
    None,
}

impl Credentials {
    pub fn from_env() -> Self {
        let auth_mount = Environment::VaultAuthMount.get();
        if let Some(token) = Environment::VaultToken.source() {
            Self::Token(token)
        } else if let Some(jwt) = Environment::VaultKubernetesJwt.source() {
            Self::Kubernetes(Kubernetes::new(
                jwt,
                Environment::VaultKubernetesRole.get(),
                auth_mount,
            ))
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
        } else if let Some(jwt) = Environment::VaultJwt.source() {
            Self::Jwt(Jwt::new(jwt, Environment::VaultJwtRole.get(), auth_mount))
        } else if let Some(certificate) = Environment::VaultCertificateName.get() {
            Self::Certificate(Certificate::new(certificate, auth_mount))
        } else {
            Self::None
        }
    }
}
