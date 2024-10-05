use crate::configuration::authentication::Credentials;
use crate::utilities::environment::Environment;

const DEFAULT_VAULT_ADDRESS: &str = "https://vault.vault.svc.cluster.local:8200";
const DEFAULT_VAULT_TRANSIT_KEY: &str = "vault-kms-provider";
const DEFAULT_TRANSIT_MOUNT_PATH: &str = "transit";

#[derive(Clone, Debug)]
pub struct VaultConfiguration {
    pub credentials: Credentials,
    pub address: String,
    pub transit_key: String,
    pub mount_path: String,
}

impl VaultConfiguration {
    pub fn new() -> Self {
        Self {
            credentials: Credentials::from_env(),
            address: Environment::VaultAddress.or(DEFAULT_VAULT_ADDRESS),
            transit_key: Environment::VaultTransitKey.or(DEFAULT_VAULT_TRANSIT_KEY),
            mount_path: Environment::VaultTransitMount.or(DEFAULT_TRANSIT_MOUNT_PATH),
        }
    }
}
