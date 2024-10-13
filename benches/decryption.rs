use criterion::BenchmarkId;
use criterion::Criterion;
use lib::configuration::socket::SocketConfiguration;
use lib::kms::key_management_service_client::KeyManagementServiceClient;
use lib::kms::{DecryptRequest, EncryptRequest};
use lib::utilities::socket::Socket;
use tokio::runtime::Runtime;
use tonic::transport::Channel;
use tonic::Request;

extern crate lib;

const BENCHMARK_NAME: &str = "vault-kms-provider";

pub async fn client() -> Result<KeyManagementServiceClient<Channel>, tonic::transport::Error> {
    let socket = Socket::default();
    let channel = socket
        .connect(&SocketConfiguration::default().socket_path)
        .await?;
    Ok(KeyManagementServiceClient::new(channel))
}

async fn decrypt(
    (encrypted, uid, key_id): &(Vec<u8>, String, String),
) -> Result<(), tonic::Status> {
    let mut client = client()
        .await
        .map_err(|e| tonic::Status::from_error(e.into()))?;
    client
        .decrypt(Request::new(DecryptRequest {
            ciphertext: encrypted.clone(),
            uid: uid.to_string(),
            key_id: key_id.to_string(),
            annotations: Default::default(),
        }))
        .await?;
    Ok(())
}

pub fn decryption_benchmark(c: &mut Criterion) {
    let mut text: Vec<u8> = vec![];
    let uid = "123".to_string();
    let mut key_id = String::new();
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let mut client = client().await.unwrap();
        let response = client
            .encrypt(Request::new(EncryptRequest {
                plaintext: "testing".as_bytes().to_vec(),
                uid: uid.to_string(),
            }))
            .await
            .unwrap()
            .into_inner();
        text = response.ciphertext.clone();
        key_id = response.key_id.clone();
    });

    c.bench_with_input(
        BenchmarkId::new(BENCHMARK_NAME, "decryption"),
        &(text, uid, key_id),
        |b, value| {
            b.to_async(&rt).iter(|| decrypt(value));
        },
    );
}
