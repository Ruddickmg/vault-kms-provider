use crate::utilities::environment::Environment;

const DEFAULT_SOCKET_PATH: &str = "./sockets/vault-kms-provider.sock";
const DEFAULT_SOCKET_PERMISSIONS: &str = "666";

pub struct SocketConfiguration {
    pub socket_path: String,
    pub permissions: String,
}

impl SocketConfiguration {
    pub fn new() -> Self {
        Self {
            socket_path: Environment::SocketPath.or(DEFAULT_SOCKET_PATH),
            permissions: Environment::SocketPermissions
                .or(DEFAULT_SOCKET_PERMISSIONS)
                .to_string(),
        }
    }
}