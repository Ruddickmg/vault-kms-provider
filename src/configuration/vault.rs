use crate::configuration::authentication::Credentials;
use crate::utilities::env::get_env;

const DEFAULT_VAULT_ADDRESS: &str = "https://vault.vault.svc.cluster.local:8200";
const DEFAULT_VAULT_TRANSIT_KEY: &str = "vault-kms-provider";
const DEFAULT_VAULT_ROLE: &str = "vault-kms-provider";
const DEFAULT_TRANSIT_MOUNT_PATH: &str = "transit";

#[derive(Clone, Debug)]
pub struct VaultConfiguration {
    pub credentials: Credentials,
    pub role: String,
    pub address: String,
    pub transit_key: String,
    pub mount_path: String,
}

impl VaultConfiguration {
    pub fn new() -> Self {
        Self {
            credentials: Credentials::from_env(),
            role: get_env("VAULT_ROLE", DEFAULT_VAULT_ROLE),
            address: get_env("VAULT_ADDRESS", DEFAULT_VAULT_ADDRESS),
            transit_key: get_env("VAULT_TRANSIT_KEY", DEFAULT_VAULT_TRANSIT_KEY),
            mount_path: get_env("VAULT_TRANSIT_PATH", DEFAULT_TRANSIT_MOUNT_PATH),
        }
    }
}
