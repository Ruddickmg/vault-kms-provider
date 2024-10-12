use crate::utilities::environment::Environment;

pub const DEFAULT_SOCKET_PATH: &str = "./sockets/vault-kms-provider.sock";
const DEFAULT_SOCKET_PERMISSIONS: &str = "666";

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod socket_configuration {
    use super::{SocketConfiguration, DEFAULT_SOCKET_PATH, DEFAULT_SOCKET_PERMISSIONS};

    #[test]
    fn initializes_values_with_default() {
        assert_eq!(
            SocketConfiguration::default(),
            SocketConfiguration {
                socket_path: DEFAULT_SOCKET_PATH.to_string(),
                permissions: DEFAULT_SOCKET_PERMISSIONS.to_string(),
            }
        );
    }

    #[test]
    fn initializes_values_with_silent() {
        assert_eq!(
            SocketConfiguration::silent(),
            SocketConfiguration {
                socket_path: DEFAULT_SOCKET_PATH.to_string(),
                permissions: DEFAULT_SOCKET_PERMISSIONS.to_string(),
            }
        );
    }
}
