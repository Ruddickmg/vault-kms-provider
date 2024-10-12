use super::client;
use criterion::{BenchmarkId, Criterion};
use lib::kms::{EncryptRequest, StatusRequest};
use tokio::runtime::Runtime;
use tonic::Request;

const BENCHMARK_NAME: &str = "vault-kms-provider";

async fn check_health() -> Result<(), std::io::Error> {
    let mut client = client()
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    client.status(Request::new(StatusRequest {})).await.unwrap();
    Ok(())
}

pub fn health_check_benchmark(c: &mut Criterion) {
    let uid = "123".to_string();
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let mut client = client().await.unwrap();
        client
            .encrypt(Request::new(EncryptRequest {
                plaintext: "testing".as_bytes().to_vec(),
                uid: uid.to_string(),
            }))
            .await
            .unwrap();
    });

    c.bench_with_input(BenchmarkId::new(BENCHMARK_NAME, "health"), &(), |b, _| {
        b.to_async(&rt).iter(|| check_health());
    });
}
