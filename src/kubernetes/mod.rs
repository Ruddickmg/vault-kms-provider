use crate::kms::api::key_management_service_client::KeyManagementServiceClient;

pub async fn client(address: String) -> Result<(), tonic::transport::Error> {
    println!("Client listening at: {}", address);
    KeyManagementServiceClient::connect(address.to_string()).await?;
    Ok(())
}
