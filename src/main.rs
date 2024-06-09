use tokio::try_join;

mod kms;
mod kubernetes;
mod vault;

const SOCKET_ADDRESS: &str = "[::1]:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = SOCKET_ADDRESS.parse()?;
    let client_address = format!("http://{}", SOCKET_ADDRESS);
    let server = tokio::spawn(async move { vault::server(server_address).await });
    let client = tokio::spawn(async move { kubernetes::client(client_address).await });
    let _ = try_join!(server, client)?;
    Ok(())
}
