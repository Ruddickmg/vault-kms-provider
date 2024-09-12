use crate::utilities::env::get_env;

const DEFAULT_VAULT_ADDRESS: &str = "https://vault.vault.cluster.svc.local:8200";
const DEFAULT_VAULT_TRANSIT_KEY: &str = "vault-kms-provider";
const DEFAULT_VAULT_TOKEN: &str = "";
const DEFAULT_VAULT_TOKEN_PATH: &str = "";
const DEFAULT_VAULT_ROLE: &str = "vault-kms-provider";

pub struct VaultConfiguration {
    pub vault_role: String,
    pub vault_address: String,
    pub vault_token_path: String,
    pub vault_token: String,
    pub vault_transit_key: String,
}

impl VaultConfiguration {
    pub fn new() -> Self {
        Self {
            vault_role: get_env("VAULT_ROLE", DEFAULT_VAULT_ROLE),
            vault_token: get_env("VAULT_TOKEN", DEFAULT_VAULT_TOKEN),
            vault_token_path: get_env("VAULT_TOKEN_PATH", DEFAULT_VAULT_TOKEN_PATH),
            vault_address: get_env("VAULT_ADDRESS", DEFAULT_VAULT_ADDRESS),
            vault_transit_key: get_env("VAULT_TRANSIT_KEY", DEFAULT_VAULT_TRANSIT_KEY),
        }
    }
}
