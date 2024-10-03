#[cfg(test)]
mod authentication {
    use lib::configuration::authentication::{AppRole, Credentials, Source, UserPass};
    use lib::configuration::vault::VaultConfiguration;
    use lib::vault::Client;
    use std::fs;
    use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

    #[tokio::test]
    async fn login_with_username_and_password() {
        let config: VaultConfiguration = VaultConfiguration {
            role: "vault-kms-provider".to_string(),
            address: "https://127.0.0.1:8400".to_string(),
            transit_key: "vault-kms-provider".to_string(),
            mount_path: "transit".to_string(),
            credentials: Credentials::UserPass(UserPass {
                username: "vault-kms-provider".to_string(),
                password: Source::Value("password".to_string()),
                mount_path: "userpass".to_string(),
            }),
        };
        let settings = VaultClientSettingsBuilder::default()
            .address(&config.address)
            .ca_certs(vec!["./test_files/certs/ca.crt".to_string()])
            .build()
            .unwrap();
        let vault_client = VaultClient::new(settings).unwrap();
        let client = Client::new(vault_client, &config);
        let result = client.get_token().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn login_with_app_role() {
        let role_id = fs::read_to_string("./test_files/role_id").unwrap();
        let secret_id = Source::FilePath("./test_files/secret_id".to_string());
        let config: VaultConfiguration = VaultConfiguration {
            role: "vault-kms-provider".to_string(),
            address: "https://127.0.0.1:8400".to_string(),
            transit_key: "vault-kms-provider".to_string(),
            mount_path: "transit".to_string(),
            credentials: Credentials::AppRole(AppRole::new(role_id, secret_id, None)),
        };
        let settings = VaultClientSettingsBuilder::default()
            .address(&config.address)
            .ca_certs(vec!["./test_files/certs/ca.crt".to_string()])
            .build()
            .unwrap();
        let vault_client = VaultClient::new(settings).unwrap();
        let client = Client::new(vault_client, &config);
        let result = client.get_token().await.map_err(|e| {
            println!("{:?}", e);
            e
        });
        assert!(result.is_ok());
    }
}
