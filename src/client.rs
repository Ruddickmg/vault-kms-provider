extern crate lib;

use lib::{
  kms::key_management_service_client::KeyManagementServiceClient,
  utilities::socket
};
use lib::configuration::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config: Configuration = Configuration::get();
  let path= config.socket_path;
  let channel = socket::connect_to_unix_socket(&path).await?;
  println!("Client listening at socket path: \"{}\"", path);
  let mut _client = KeyManagementServiceClient::new(channel);
  // client.encrypt(Request::new(EncryptRequest {
  //   plaintext: "hello!".as_bytes().to_vec(),
  //   uid: "123".to_string(),
  // })).await?;
  Ok(())
}
