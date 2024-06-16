use crate::utilities::env::get_env;

const DEFAULT_VAULT_ADDRESS: &str = "https://vault.vault.cluster.svc.local:8200";
const DEFAULT_SOCKET_PATH: &str = "./sockets/vault-kms-provider.socket";
const DEFAULT_VAULT_TRANSIT_KEY: &str = "kms";
const DEFAULT_VAULT_TOKEN: &str = "";

pub struct Configuration {
  pub socket_path: String,
  pub vault_address: String,
  pub vault_token: String,
  pub vault_transit_key: String,
}

impl Configuration {
  pub fn get() -> Configuration {
    Configuration {
      vault_token: get_env("VAULT_TOKEN", DEFAULT_VAULT_TOKEN),
      vault_address: get_env("VAULT_ADDRESS", DEFAULT_VAULT_ADDRESS),
      vault_transit_key: get_env("VAULT_TRANSIT_KEY", DEFAULT_VAULT_TRANSIT_KEY),
      socket_path: get_env("SOCKET_PATH", DEFAULT_SOCKET_PATH),
    }
  }
}
