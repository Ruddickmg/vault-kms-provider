#[cfg(test)]
mod vault_integration_tests {
  use tokio;
  use tonic::Request;
  use lib;
  use lib::kms::{DecryptRequest, EncryptRequest, StatusRequest};

  #[tokio::test]
  async fn encryption_and_decryption() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = lib::client().await?;
    let text = "hello world!";
    let uid = "123";
    let response = client
      .encrypt(Request::new(EncryptRequest {
        plaintext: text.as_bytes().to_vec(),
        uid: uid.to_string(),
      }))
      .await?.into_inner();
    let decrypt_resp = client
      .decrypt(Request::new(DecryptRequest {
        ciphertext: response.ciphertext,
        uid: uid.to_string(),
        key_id: response.key_id.clone(),
        annotations: response.annotations.clone(),
      }))
      .await?;
    let decrypted = String::from_utf8(decrypt_resp.into_inner().plaintext).unwrap();
    assert_eq!(&decrypted, text);
    Ok(())
  }

  #[tokio::test]
  async fn vault_status() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = lib::client().await?;
    let status_resp = client.status(Request::new(StatusRequest {})).await?;
    assert_eq!(status_resp.into_inner().healthz, "ok");
    Ok(())
  }
}

