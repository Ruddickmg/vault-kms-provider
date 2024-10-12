mod common;

#[cfg(test)]
mod authentication {
    use super::common;
    use lib::configuration::authentication::{AppRole, Certificate, Credentials, Jwt, UserPass};
    use lib::utilities::logging;
    use lib::utilities::source::Source;
    use lib::vault::Client;
    use std::fs;
    use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

    async fn test_login_with_credentials(credentials: Credentials) {
        let mut client_config = common::server_config();
        client_config.vault.credentials = credentials;
        let settings = VaultClientSettingsBuilder::default()
            .address(&client_config.vault.address.clone())
            .identity(client_config.tls.identity())
            .ca_certs(client_config.tls.certs())
            .build()
            .unwrap();
        let vault_client = VaultClient::new(settings).unwrap();
        let client = Client::new(vault_client, &client_config.vault);

        let result = client.get_token().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn login_with_username_and_password() {
        test_login_with_credentials(Credentials::UserPass(UserPass {
            username: "vault-kms-provider".to_string(),
            password: Source::Value("password".to_string()),
            mount_path: "userpass".to_string(),
        }))
        .await;
    }

    #[tokio::test]
    async fn login_with_app_role() {
        logging::initialize();
        let role_id = fs::read_to_string("./test_files/role_id")
            .unwrap()
            .trim()
            .to_string();
        let secret_id = fs::read_to_string("./test_files/secret_id")
            .unwrap()
            .trim()
            .to_string();
        test_login_with_credentials(Credentials::AppRole(AppRole::new(
            role_id,
            Source::Value(secret_id),
            None,
        )))
        .await;
    }

    #[tokio::test]
    async fn login_with_jwt() {
        let jwt = fs::read_to_string("./test_files/jwt/token").unwrap();
        test_login_with_credentials(Credentials::Jwt(Jwt::new(
            Source::Value(jwt.to_string()),
            Some("vault-kms-provider".to_string()),
            None,
        )))
        .await;
    }

    #[tokio::test]
    async fn login_with_certificate() {
        test_login_with_credentials(Credentials::Certificate(Certificate::new(
            "vault-kms-provider".to_string(),
            None,
        )))
        .await;
    }
}
