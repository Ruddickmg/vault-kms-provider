use tokio::try_join;

mod vault;
mod kms;
mod kubernetes;

const SOCKET_ADDRESS: &str = "[::1]:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let server_address = SOCKET_ADDRESS.parse()?;
  let client_address = format!("http://{}", "localhost:8080");
  let server = tokio::spawn(async move { vault::server(server_address).await });
  let client = tokio::spawn(async move { kubernetes::client(client_address).await });
  try_join!(server, client)?;
  Ok(())
}
