use crate::utilities::environment::Environment;
use crate::utilities::socket::UnixSocketPermissions;

const DEFAULT_SOCKET_PATH: &str = "./sockets/vault-kms-provider.sock";
const DEFAULT_SOCKET_PERMISSIONS: &str = "any";

pub struct SocketConfiguration {
    pub socket_path: String,
    pub permissions: UnixSocketPermissions,
}

impl SocketConfiguration {
    pub fn new() -> Self {
        Self {
            socket_path: Environment::SocketPath.or(DEFAULT_SOCKET_PATH),
            permissions: Environment::SocketPermissions
                .or(DEFAULT_SOCKET_PERMISSIONS)
                .into(),
        }
    }
}
