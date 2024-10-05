#[cfg(test)]
mod authentication {
    use lib::configuration::authentication::{AppRole, Certificate, Credentials, Jwt, UserPass};
    use lib::configuration::tls;
    use lib::configuration::vault::VaultConfiguration;
    use lib::utilities::source::Source;
    use lib::vault::Client;
    use std::fs;
    use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
    use vaultrs::error::ClientError;

    async fn login_with_credentials(credentials: Credentials) -> Result<String, ClientError> {
        let config: VaultConfiguration = VaultConfiguration {
            address: "https://127.0.0.1:8400".to_string(),
            transit_key: "vault-kms-provider".to_string(),
            mount_path: "transit".to_string(),
            credentials,
        };
        let tls_config = tls::TlsConfiguration {
            cert: Some("./test_files/certs/tls.crt".to_string()),
            key: Some("./test_files/certs/tls.key".to_string()),
            ca: Some("./test_files/certs/ca.crt".to_string()),
            directory: None,
        };
        let settings = VaultClientSettingsBuilder::default()
            .address(&config.address)
            .identity(tls_config.identity())
            .ca_certs(tls_config.certs())
            .build()
            .unwrap();
        let vault_client = VaultClient::new(settings).unwrap();
        let client = Client::new(vault_client, &config);
        client.get_token().await.map_err(|error| {
            println!("Error retrieving token: {:?}", error);
            error
        })
    }

    #[tokio::test]
    async fn login_with_username_and_password() {
        let result = login_with_credentials(Credentials::UserPass(UserPass {
            username: "vault-kms-provider".to_string(),
            password: Source::Value("password".to_string()),
            mount_path: "userpass".to_string(),
        }))
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn login_with_app_role() {
        let role_id = fs::read_to_string("./test_files/role_id").unwrap();
        let secret_id = Source::FilePath("./test_files/secret_id".to_string());
        let result =
            login_with_credentials(Credentials::AppRole(AppRole::new(role_id, secret_id, None)))
                .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn login_with_jwt() {
        let jwt = fs::read_to_string("./test_files/jwt/token").unwrap();
        let result = login_with_credentials(Credentials::Jwt(Jwt::new(
            Source::Value(jwt.to_string()),
            Some("vault-kms-provider".to_string()),
            None,
        )))
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn login_with_certificate() {
        let result = login_with_credentials(Credentials::Certificate(Certificate::new(
            "vault-kms-provider".to_string(),
            None,
        )))
        .await;

        assert!(result.is_ok());
    }
}
