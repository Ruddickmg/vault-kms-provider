use futures::lock;
use lib::configuration::authentication::Credentials;
use lib::configuration::health::HealthCheckConfiguration;
use lib::configuration::socket::SocketConfiguration;
use lib::configuration::tls::TlsConfiguration;
use lib::configuration::vault::VaultConfiguration;
use lib::configuration::{tls, ServerConfiguration};
use lib::kms::{
    key_management_service_client::KeyManagementServiceClient,
    key_management_service_server::KeyManagementServiceServer,
};
use lib::server;
use lib::utilities::socket::Socket;
use lib::utilities::source::Source;
use log::debug;
use std::ffi::OsString;
use std::future::Future;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::OnceLock;
use tokio::select;
use tonic::transport::Channel;
use uuid::Uuid;

pub fn setup() {
    println!("Setting up integration tests");
}

static COUNTER: AtomicU16 = AtomicU16::new(0);

pub fn server_config() -> ServerConfiguration {
    let id = Uuid::new_v4().to_string();
    let num = COUNTER.fetch_add(1, Ordering::Relaxed);
    ServerConfiguration {
        health: HealthCheckConfiguration {
            endpoint: format!("127.0.0.1:808{}", num),
        },
        socket: SocketConfiguration {
            socket_path: format!("@test_files/kms-{}.sock", id),
            permissions: "777".to_string(),
        },
        vault: VaultConfiguration {
            address: "https://localhost:8400".to_string(),
            transit_key: "vault-kms-provider".to_string(),
            mount_path: "transit".to_string(),
            credentials: Credentials::Token(Source::Value("SiQOECxwSDCeQt1r0n5kqQCr".to_string())),
        },
        tls: TlsConfiguration {
            cert: Some("./test_files/certs/tls.crt".to_string()),
            key: Some("./test_files/certs/tls.key".to_string()),
            ca: Some("./test_files/certs/ca.crt".to_string()),
            directory: None,
        },
    }
}

pub async fn client(
    socket_path: &str,
    lock: &'static OnceLock<OsString>,
) -> Result<KeyManagementServiceClient<Channel>, tonic::transport::Error> {
    let socket = Socket::with_path(lock);
    let channel = socket.connect(socket_path).await?;
    Ok(KeyManagementServiceClient::new(channel))
}

pub async fn run_against_server<F, Fut>(config: ServerConfiguration, test: F)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    select! {
        r = async {
            server(config).await.map_err(|e| {
                debug!("Server Error!: {}", e.to_string());
                e
            }).unwrap();
        } => r,
        r = async {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            test().await;
        } => r,
    }
}
