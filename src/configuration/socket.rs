use crate::utilities::environment::Environment;

pub const DEFAULT_SOCKET_PATH: &str = "./sockets/vault-kms-provider.sock";
const DEFAULT_SOCKET_PERMISSIONS: &str = "666";

pub struct SocketConfiguration {
    pub socket_path: String,
    pub permissions: String,
}

impl Default for SocketConfiguration {
    fn default() -> Self {
        Self {
            socket_path: Environment::SocketPath.or(DEFAULT_SOCKET_PATH),
            permissions: Environment::SocketPermissions
                .or(DEFAULT_SOCKET_PERMISSIONS)
                .to_string(),
        }
    }
}

impl SocketConfiguration {
    pub fn silent() -> Self {
        Self {
            socket_path: Environment::SocketPath.silent_or(DEFAULT_SOCKET_PATH),
            permissions: Environment::SocketPermissions
                .silent_or(DEFAULT_SOCKET_PERMISSIONS)
                .to_string(),
        }
    }
}
