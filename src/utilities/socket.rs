use hyper_util::rt::TokioIo;
use std::ffi::{OsStr, OsString};
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
    path: &'static OnceLock<OsString>,
    permissions: Permissions,
}

impl Default for Socket {
    fn default() -> Self {
        Self::new("666", &UNIX_SOCKET_PATH)
    }
}

impl Socket {
    pub fn new(permissions: &str, path: &'static OnceLock<OsString>) -> Self {
        Self {
            path,
            permissions: Permissions::from_mode(
                u32::from_str_radix(permissions, 8)
                    .ok()
                    .unwrap_or(DEFAULT_SOCKET_PERMISSIONS),
            ),
        }
    }
    pub fn with_permissions(permissions: &str) -> Self {
        Self::new(permissions, &UNIX_SOCKET_PATH)
    }

    pub fn with_path(path: &'static OnceLock<OsString>) -> Self {
        Self::new("666", path)
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
    pub async fn connect(&self, path: &str) -> Result<Channel, tonic::transport::Error> {
        info!("Server listening to unix socket: \"{}\"", path);
        self.path.get_or_init(|| Self::format_path(path));
        // this url doesn't matter since we are replacing it with the unix stream connection
        Endpoint::try_from("http://[::]:50051")?
            .connect_with_connector(service_fn(|_| async {
                Ok::<_, std::io::Error>(TokioIo::new(
                    UnixStream::connect(self.path.get().unwrap()).await?,
                ))
            }))
            .await
    }
}

#[cfg(test)]
mod socket_permissions {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::time::Duration;
    static TEST_ABSTRACT_SOCKET_PATH: OnceLock<OsString> = OnceLock::new();
    static TEST_ABSTRACT_SOCKET_PATH2: OnceLock<OsString> = OnceLock::new();
    static TEST_SOCKET_FILE_PATH: OnceLock<OsString> = OnceLock::new();

    #[test]
    fn can_parse_a_string_into_a_permissions_u32() {
        let socket = Socket::with_permissions("777");
        assert_eq!(Permissions::from_mode(0o777), socket.permissions);
    }

    #[tokio::test]
    async fn can_create_and_connect_to_abstract_sockets() {
        let path: &str = "@test_files/abstract_test.sock";
        let socket = Socket::with_path(&TEST_ABSTRACT_SOCKET_PATH);
        let _stream = socket.listen(path).unwrap();
        let result = socket.connect(path).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn does_not_create_abstract_sockets_on_filesystem() {
        let path: &str = "@test_files/abstract_test2.sock";
        let socket = Socket::with_path(&TEST_ABSTRACT_SOCKET_PATH2);
        let _stream = socket.listen(path).unwrap();
        let _result = socket.connect(path).await;
        assert!(!fs::exists(path).unwrap());
    }

    #[tokio::test]
    async fn can_create_and_connect_to_file_sockets() {
        let path: &str = "./test_files/test.sock";
        let socket = Socket::with_path(&TEST_SOCKET_FILE_PATH);
        let _stream = socket.listen(path).unwrap();
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let result = socket.connect(path).await;
        assert!(result.is_ok());
    }
}
