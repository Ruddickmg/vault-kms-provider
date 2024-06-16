use tokio::net::{UnixStream, UnixListener};
use tonic::transport::{Channel, Endpoint};
use std::path::Path;
use std::sync::OnceLock;
use tokio_stream::wrappers::UnixListenerStream;
#[cfg(unix)]
use tower::service_fn;

static UNIX_SOCKET_PATH: OnceLock<String> = OnceLock::new();

pub fn create_unix_socket(path: &str) -> Result<UnixListenerStream, Box<dyn std::error::Error>> {
  let directory_path = Path::new(path).parent().unwrap();
  std::fs::create_dir_all(directory_path)?;
  let uds = UnixListener::bind(path)?;
  Ok(UnixListenerStream::new(uds))
}

pub async fn connect_to_unix_socket(path: &str) -> Result<Channel, tonic::transport::Error>  {
  UNIX_SOCKET_PATH.set(path.to_string()).unwrap();
  // this url doesn't matter since we are replacing it with the unix stream connection
  Endpoint::try_from("[::1]:50051")?
    .connect_with_connector(service_fn(|_| async {
      Ok::<_, std::io::Error>(UnixStream::connect(UNIX_SOCKET_PATH.get().unwrap()).await?)
    }))
    .await
}
