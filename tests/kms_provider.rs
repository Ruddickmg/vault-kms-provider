#[cfg(test)]
mod encryption_and_decryption {
    use lib;
    use lib::kms::{DecryptRequest, EncryptRequest};
    use tokio;
    use tonic::Request;

    #[tokio::test]
    async fn can_encrypt_and_decrypt_messages() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = lib::client().await?;
        let text = "hello world!";
        let uid = "123";
        let response = client
            .encrypt(Request::new(EncryptRequest {
                plaintext: text.as_bytes().to_vec(),
                uid: uid.to_string(),
            }))
            .await?
            .into_inner();
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
}

#[cfg(test)]
mod status {
    use lib::kms::{EncryptRequest, StatusRequest};
    use tonic::Request;

    #[tokio::test]
    async fn returns_ok_status_when_key_exists() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = lib::client().await?;
        let text = "hello world!";
        let uid = "123";
        client
            .encrypt(Request::new(EncryptRequest {
                plaintext: text.as_bytes().to_vec(),
                uid: uid.to_string(),
            }))
            .await?;
        let status_resp = client.status(Request::new(StatusRequest {})).await?;

        assert_eq!(status_resp.into_inner().healthz, "ok");
        Ok(())
    }
}
