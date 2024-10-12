use criterion::{criterion_group, criterion_main};
use decryption::decryption_benchmark;
use encryption::encryption_benchmark;
use health::health_check_benchmark;
use lib::configuration::socket::SocketConfiguration;
use lib::kms::key_management_service_client::KeyManagementServiceClient;
use lib::utilities::socket::Socket;
use tonic::transport::Channel;

pub async fn client() -> Result<KeyManagementServiceClient<Channel>, tonic::transport::Error> {
    let socket = Socket::default();
    let channel = socket
        .connect(&SocketConfiguration::default().socket_path)
        .await?;
    Ok(KeyManagementServiceClient::new(channel))
}

mod decryption;
mod encryption;
mod health;

criterion_group!(
    benches,
    decryption_benchmark,
    encryption_benchmark,
    health_check_benchmark
);
criterion_main!(benches);
