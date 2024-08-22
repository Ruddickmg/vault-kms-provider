use crate::utilities::env::get_env;
use crate::utilities::socket::UnixSocketPermissions;

const DEFAULT_VAULT_ADDRESS: &str = "https://vault.vault.cluster.svc.local:8200";
const DEFAULT_SOCKET_PATH: &str = "./sockets/vault-kms-provider.sock";
const DEFAULT_SOCKET_PERMISSIONS: &str = "any";
const DEFAULT_VAULT_TRANSIT_KEY: &str = "vault-kms-provider";
const DEFAULT_VAULT_TOKEN: &str = "";
const DEFAULT_VAULT_TOKEN_PATH: &str = "";
const DEFAULT_HEALTH_ENDPOINT: &str = "0.0.0.0:8080";

pub struct HealthCheckConfiguration {
    pub endpoint: String,
}

pub struct VaultConfiguration {
    pub vault_address: String,
    pub vault_token_path: String,
    pub vault_token: String,
    pub vault_transit_key: String,
}

pub struct SocketConfiguration {
    pub socket_path: String,
    pub permissions: UnixSocketPermissions,
}

pub fn health_check_endpoint() -> HealthCheckConfiguration {
    HealthCheckConfiguration {
        endpoint: get_env("HTTP_ADDRESS", DEFAULT_HEALTH_ENDPOINT),
    }
}

pub fn socket() -> SocketConfiguration {
    SocketConfiguration {
        socket_path: get_env("SOCKET_PATH", DEFAULT_SOCKET_PATH),
        permissions: get_env("SOCKET_PERMISSIONS", DEFAULT_SOCKET_PERMISSIONS).into(),
    }
}

pub fn vault() -> VaultConfiguration {
    VaultConfiguration {
        vault_token: get_env("VAULT_TOKEN", DEFAULT_VAULT_TOKEN),
        vault_token_path: get_env("VAULT_TOKEN_PATH", DEFAULT_VAULT_TOKEN_PATH),
        vault_address: get_env("VAULT_ADDRESS", DEFAULT_VAULT_ADDRESS),
        vault_transit_key: get_env("VAULT_TRANSIT_KEY", DEFAULT_VAULT_TRANSIT_KEY),
    }
}
