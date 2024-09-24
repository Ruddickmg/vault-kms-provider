use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use tokio::runtime::Runtime;
use tonic::Request;
use lib::kms::{DecryptRequest, EncryptRequest, StatusRequest};
use fake::{Fake, faker::name::raw::*};
use fake::locales::EN;

extern crate lib;

async fn make_calls_to_vault_for_encryption(text: &str) -> Result<(), std::io::Error> {
  let mut client = lib::client().await.map_err(|e|std::io::Error::other(e.to_string()))?;
  let uid = "123";
  let response = client
    .encrypt(Request::new(EncryptRequest {
      plaintext: text.as_bytes().to_vec(),
      uid: uid.to_string(),
    }))
    .await.map_err(|e|std::io::Error::other(e.to_string()))?
    .into_inner();
  client
    .decrypt(Request::new(DecryptRequest {
      ciphertext: response.ciphertext,
      uid: uid.to_string(),
      key_id: response.key_id.clone(),
      annotations: response.annotations.clone(),
    }))
    .await.map_err(|e|std::io::Error::other(e.to_string()))?;
  client.status(Request::new(StatusRequest {})).await.map_err(|e|std::io::Error::other(e.to_string()))?;
  Ok(())
}

fn from_elem(c: &mut Criterion) {
  let size: usize = 1024;

  c.bench_with_input(BenchmarkId::new("input_example", size), &size, |b, &s| {
    let text: String = Name(EN).fake();
    let rt = Runtime::new().unwrap();

    b.to_async(rt).iter(|| make_calls_to_vault_for_encryption(&text));
  });
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
