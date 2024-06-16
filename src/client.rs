extern crate lib;

use tonic::Request;
use lib::{
    configuration, kms::key_management_service_client::KeyManagementServiceClient,
    utilities::socket,
};
use lib::kms::{DecryptRequest, DecryptResponse, EncryptRequest, StatusRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = configuration::socket();
    let path = config.socket_path;
    let channel = socket::connect_to_unix_socket(&path).await?;
    println!("Client listening at socket path: \"{}\"", path);
    let mut client = KeyManagementServiceClient::new(channel);
    // client.status(Request::new(StatusRequest {})).await?;
    let encrypt_resp = client.encrypt(Request::new(EncryptRequest {
      plaintext: "hello!".as_bytes().to_vec(),
      uid: "123".to_string(),
    })).await?;
    let encrypt_req = encrypt_resp.into_inner();
    let encrypted = encrypt_req.clone().ciphertext;
    println!("encrypted: {}", String::from_utf8(encrypted.clone()).unwrap());
    let decrypt_resp = client.decrypt(Request::new(DecryptRequest {
        ciphertext: encrypted,
        uid: "123".to_string(),
        key_id: encrypt_req.key_id.clone(),
        annotations: encrypt_req.annotations.clone(),
    })).await?;
    println!("decrypted: {}", String::from_utf8(decrypt_resp.into_inner().plaintext).unwrap());
    let status_resp = client.status(Request::new(StatusRequest {}))
      .await?;
    println!("status: {:#?}", status_resp.into_inner());
    Ok(())
}
