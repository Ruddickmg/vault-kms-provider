use crate::configuration::authentication::Authentication;
use crate::utilities::env::get_env;

const DEFAULT_VAULT_ADDRESS: &str = "https://vault.vault.svc.cluster.local:8200";
const DEFAULT_VAULT_TRANSIT_KEY: &str = "vault-kms-provider";
const DEFAULT_VAULT_ROLE: &str = "vault-kms-provider";

#[derive(Clone, Debug)]
pub struct VaultConfiguration {
    pub auth: Authentication,
    pub role: String,
    pub address: String,
    pub transit_key: String,
}

impl VaultConfiguration {
    pub fn new() -> Self {
        Self {
            auth: Authentication::get(),
            role: get_env("VAULT_ROLE", DEFAULT_VAULT_ROLE),
            address: get_env("VAULT_ADDRESS", DEFAULT_VAULT_ADDRESS),
            transit_key: get_env("VAULT_TRANSIT_KEY", DEFAULT_VAULT_TRANSIT_KEY),
        }
    }
}
