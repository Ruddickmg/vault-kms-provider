use hyper_util::rt::TokioIo;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::OnceLock;
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Channel, Endpoint};
#[cfg(unix)]
use tower::service_fn;

static UNIX_SOCKET_PATH: OnceLock<String> = OnceLock::new();

#[derive(Clone, Copy)]
pub enum UnixSocketPermissions {
    OwnerAccess = 0o600,
    OwnerAndGroupAccess = 0o660,
    Any = 0o666,
}

impl From<&str> for UnixSocketPermissions {
    fn from(value: &str) -> Self {
        match value {
            "user" => UnixSocketPermissions::OwnerAccess,
            "group" => UnixSocketPermissions::OwnerAndGroupAccess,
            _ => UnixSocketPermissions::Any,
        }
    }
}

impl From<String> for UnixSocketPermissions {
    fn from(value: String) -> Self {
        (&value).as_str().into()
    }
}

pub fn create_unix_socket(
    path: &str,
    permissions: UnixSocketPermissions,
) -> Result<UnixListenerStream, Box<dyn std::error::Error>> {
    let file_path = Path::new(path);
    if file_path.exists() {
        std::fs::remove_file(file_path)?;
    }
    let directory_path = file_path.parent().unwrap();
    std::fs::create_dir_all(directory_path)?;
    let uds = UnixListener::bind(path)?;
    let unix_stream = UnixListenerStream::new(uds);
    std::fs::set_permissions(file_path, Permissions::from_mode(permissions as u32))?;
    Ok(unix_stream)
}

pub async fn connect_to_unix_socket(path: &str) -> Result<Channel, tonic::transport::Error> {
    UNIX_SOCKET_PATH.get_or_init(|| path.to_string());
    // this url doesn't matter since we are replacing it with the unix stream connection
    Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_| async {
            Ok::<_, std::io::Error>(TokioIo::new(
                UnixStream::connect(UNIX_SOCKET_PATH.get().unwrap()).await?,
            ))
        }))
        .await
}
