use grpc::{ClientStub, ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};
use grpc_compiler;
mod api_grpc;
mod vault;

use api_grpc::KeyManagementServiceClient;
use crate::api_grpc::{KeyManagementServiceServer};

fn main() {
    let host = "localhost";
    let port = 8200;
    let vaultKms = vault::kms::VaultKms::new();
    let Ok(client) = grpc::ClientBuilder::new(host, port)
      .build();
    let kmsClient = KeyManagementServiceClient::with_client(client.into());
    let kmxServer = KeyManagementServiceServer::new_service_def(vaultKms);

    println!("Hello, world!");
}
