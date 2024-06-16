extern crate lib;

use tonic::transport::Server;
use lib::{
  vault,
  configuration::Configuration,
  kms::key_management_service_server::KeyManagementServiceServer,
  utilities::socket::create_unix_socket
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let configuration = Configuration::get();
  let vault_kms_server =
    vault::VaultKmsServer::new("kms", &configuration.vault_address, "token");
  println!("Server listening to socket @\"{}\", connecting to vault @\"{}\"", configuration.socket_path, configuration.vault_address);
  Server::builder()
    .add_service(KeyManagementServiceServer::new(vault_kms_server))
    .serve_with_incoming(create_unix_socket(&configuration.socket_path)?)
    .await?;
  Ok(())
}
