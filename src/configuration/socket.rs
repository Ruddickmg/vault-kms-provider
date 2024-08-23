use crate::utilities::env::get_env;
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
            socket_path: get_env("SOCKET_PATH", DEFAULT_SOCKET_PATH),
            permissions: get_env("SOCKET_PERMISSIONS", DEFAULT_SOCKET_PERMISSIONS).into(),
        }
    }
}

