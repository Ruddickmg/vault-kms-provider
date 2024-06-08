use api_grpc::{ KeyManagementServiceClient, KeyManagementServiceServer };
use grpc::ClientStub;
use vault::api;

mod api_grpc;
mod vault;

fn main() {
    let host = "localhost";
    let port = 8200;
    let vault_kms = vault::kms::VaultKms::new("kms", "https://vault.vault.svc.cluster.local:8200", "abcd");
    let Ok(client) = grpc::ClientBuilder::new(host, port)
      .build();
    let _kms_client = KeyManagementServiceClient::with_client(client.into());
    let _kms_server = KeyManagementServiceServer::new_service_def(vault_kms);

    println!("Hello, world!");
}
