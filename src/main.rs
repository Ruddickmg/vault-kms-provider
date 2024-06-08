mod vault;
mod kms;

use tonic::{transport::Server};
use tonic::server::ClientStreamingService;
use vault::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse()?;
  let vault_kms_server = server::VaultKmsServer::new();
  Server::builder()
    .add_service(vault_kms_server)
    .serve(addr)
    .await?;

  Ok(())
}
