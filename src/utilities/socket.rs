use std::ffi::{OsStr, OsString};
use hyper_util::rt::TokioIo;
use std::fs::Permissions;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::OnceLock;
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Channel, Endpoint};
#[cfg(unix)]
use tower::service_fn;
use tracing::{debug, info, instrument};

static UNIX_SOCKET_PATH: OnceLock<OsString> = OnceLock::new();

const DEFAULT_SOCKET_PERMISSIONS: u32 = 0o666;

#[derive(Debug, Clone)]
pub struct Socket {
    permissions: Permissions,
}

impl From<&str> for Socket {
    fn from(value: &str) -> Self {
        Self {
            permissions: Permissions::from_mode(
                u32::from_str_radix(value, 8)
                    .ok()
                    .unwrap_or(DEFAULT_SOCKET_PERMISSIONS),
            ),
        }
    }
}

impl From<String> for Socket {
    fn from(value: String) -> Self {
        (&value).as_str().into()
    }
}

impl Socket {
    pub fn new(permissions: &str) -> Self {
        Self::from(permissions)
    }

    fn format_path(path: &str) -> OsString {
        if path.starts_with("@") {
            let bytes = path.as_bytes()[1..].to_vec();
            let mut null_bytes = b"\0".to_vec();
            null_bytes.extend_from_slice(bytes.as_slice());
            OsStr::from_bytes(null_bytes.as_slice()).to_os_string()
        } else {
            OsString::from(path)
        }
    }

    #[instrument]
    pub fn listen(&self, path: &str) -> Result<UnixListenerStream, Box<dyn std::error::Error>> {
        let is_abstract: bool = path.starts_with("@") || path.as_bytes().starts_with(b"\0");
        let os_path = Self::format_path(path);
        let file_path = Path::new(&os_path);

        if is_abstract {
            debug!("Creating abstract socket, skipping file creation");
        } else {
            debug!("Creating socket file");
            if file_path.exists() && !is_abstract {
                std::fs::remove_file(file_path)?;
            }
            let directory_path = file_path.parent().unwrap();
            std::fs::create_dir_all(directory_path)?;
        }

        let uds = UnixListener::bind(file_path)?;
        let unix_stream = UnixListenerStream::new(uds);
        if !is_abstract {
            std::fs::set_permissions(file_path, self.permissions.clone())?;
        }
        Ok(unix_stream)
    }

    #[instrument]
    pub async fn connect(path: &str) -> Result<Channel, tonic::transport::Error> {
        info!("Server listening to unix socket: \"{}\"", path);
        UNIX_SOCKET_PATH.get_or_init(|| Self::format_path(path));
        // this url doesn't matter since we are replacing it with the unix stream connection
        Endpoint::try_from("http://[::]:50051")?
            .connect_with_connector(service_fn(|_| async {
                Ok::<_, std::io::Error>(TokioIo::new(
                    UnixStream::connect(UNIX_SOCKET_PATH.get().unwrap()).await?,
                ))
            }))
            .await
    }
}

#[cfg(test)]
mod socket_permissions {
    use std::fs;
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn can_parse_a_string_into_a_permissions_u32() {
        let socket = Socket::from("777");
        assert_eq!(Permissions::from_mode(0o777), socket.permissions);
    }

    #[tokio::test]
    async fn can_create_and_connect_to_abstract_sockets() {
        let path: &str = "@test_files/abstract_test.sock";
        let socket = Socket::new("777");
        let _stream = socket.listen(path).unwrap();
        let result = Socket::connect(path).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn does_not_create_abstract_sockets_on_filesystem() {
        let path: &str = "@test_files/abstract_test2.sock";
        let socket = Socket::new("777");
        let _stream = socket.listen(path).unwrap();
        let _result = Socket::connect(path).await;
        assert!(!fs::exists(path).unwrap());
    }

    #[tokio::test]
    async fn can_create_and_connect_to_file_sockets() {
        let path: &str = "./test_files/test.sock";
        let socket = Socket::new("777");
        let _stream = socket.listen(path).unwrap();
        let result = Socket::connect(path).await;
        assert!(result.is_ok());
    }
}
