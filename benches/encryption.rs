use criterion::{BenchmarkId, Criterion};
use lib::configuration::socket::SocketConfiguration;
use lib::kms::key_management_service_client::KeyManagementServiceClient;
use lib::kms::EncryptRequest;
use lib::utilities::socket::Socket;
use tokio::runtime::Runtime;
use tonic::transport::Channel;
use tonic::Request;

const BENCHMARK_NAME: &str = "vault-kms-provider";

pub async fn client() -> Result<KeyManagementServiceClient<Channel>, tonic::transport::Error> {
    let socket = Socket::default();
    let channel = socket
        .connect(&SocketConfiguration::default().socket_path)
        .await?;
    Ok(KeyManagementServiceClient::new(channel))
}

async fn encrypt((text, uid): &(Vec<u8>, String)) -> Result<(), tonic::Status> {
    let mut client = client()
        .await
        .map_err(|e| tonic::Status::from_error(e.into()))?;
    client
        .encrypt(Request::new(EncryptRequest {
            plaintext: text.clone(),
            uid: uid.to_string(),
        }))
        .await?;
    Ok(())
}

pub fn encryption_benchmark(c: &mut Criterion) {
    let text: Vec<u8> = "testing".as_bytes().to_vec();
    let uid = "123".to_string();
    let rt = Runtime::new().unwrap();

    c.bench_with_input(
        BenchmarkId::new(BENCHMARK_NAME, "encryption"),
        &(text, uid),
        |b, value| {
            b.to_async(&rt).iter(|| encrypt(value));
        },
    );
}
