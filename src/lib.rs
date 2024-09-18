use crate::configuration::socket::SocketConfiguration;
use crate::kms::key_management_service_client::KeyManagementServiceClient;
use crate::utilities::socket;
use tonic::transport::Channel;

pub mod configuration;
pub mod utilities;
pub mod vault;
pub mod kms {
    tonic::include_proto!("v2");
}

pub async fn client() -> Result<KeyManagementServiceClient<Channel>, tonic::transport::Error> {
    let config = SocketConfiguration::new();
    let channel = socket::connect_to_unix_socket(&config.socket_path).await?;
    Ok(KeyManagementServiceClient::new(channel))
}
