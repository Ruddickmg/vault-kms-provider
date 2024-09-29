#[cfg(test)]
mod authentication {
  use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
  use lib::configuration::authentication::{Authentication, Credentials};
  use lib::configuration::vault::VaultConfiguration;
  use lib::vault::Client;
  use lib::utilities::logging;

  #[tokio::test]
  async fn login_with_username_and_password() {
    logging::initialize();
    let config: VaultConfiguration = VaultConfiguration {
      auth: Authentication::Credentials(Credentials {
        username: "vault-kms-provider".to_string(),
        password: "password".to_string(),
        path: "userpass".to_string(),
      }),
      role: "vault-kms-provider".to_string(),
      address: "https://127.0.0.1:8400".to_string(),
      transit_key: "vault-kms-provider".to_string(),
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
}