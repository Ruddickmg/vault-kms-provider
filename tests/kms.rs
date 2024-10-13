mod common;

#[cfg(test)]
mod encryption_and_decryption {
    use super::common;
    use lib;
    use lib::kms::{DecryptRequest, EncryptRequest};
    use std::ffi::OsString;
    use std::sync::OnceLock;
    use tokio;
    use tonic::Request;

    static UNIX_SOCKET_PATH: OnceLock<OsString> = OnceLock::new();

    #[tokio::test]
    async fn can_encrypt_and_decrypt_messages() {
        let config = common::server_config();
        let socket_path = config.socket.socket_path.clone();
        common::run_against_server(config, || async {
            let mut client = common::client(&socket_path, &UNIX_SOCKET_PATH)
                .await
                .unwrap();
            let text = "hello world!";
            let uid = "123";
            let response = client
                .encrypt(Request::new(EncryptRequest {
                    plaintext: text.as_bytes().to_vec(),
                    uid: uid.to_string(),
                }))
                .await
                .unwrap()
                .into_inner();
            let decrypt_resp = client
                .decrypt(Request::new(DecryptRequest {
                    ciphertext: response.ciphertext,
                    uid: uid.to_string(),
                    key_id: response.key_id.clone(),
                    annotations: response.annotations.clone(),
                }))
                .await
                .unwrap();
            let decrypted = String::from_utf8(decrypt_resp.into_inner().plaintext).unwrap();
            assert_eq!(&decrypted, text);
        })
        .await;
    }
}

#[cfg(test)]
mod status {
    use crate::common;
    use lib::kms::StatusRequest;
    use lib::utilities::logging;
    use std::ffi::OsString;
    use std::sync::OnceLock;
    use tonic::Request;

    static UNIX_SOCKET_PATH: OnceLock<OsString> = OnceLock::new();

    #[tokio::test]
    async fn returns_ok_status_when_queried() {
        logging::initialize();
        let config = common::server_config();
        let socket_path = config.socket.socket_path.clone();
        common::run_against_server(config, || async {
            let mut client = common::client(&socket_path, &UNIX_SOCKET_PATH)
                .await
                .unwrap();
            let status_resp = client.status(Request::new(StatusRequest {})).await.unwrap();

            assert_eq!(status_resp.into_inner().healthz, "ok");
        })
        .await;
    }
}
