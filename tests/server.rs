#[cfg(test)]
mod server {
    use lib::configuration::authentication::Credentials;
    use lib::configuration::health::HealthCheckConfiguration;
    use lib::configuration::socket::SocketConfiguration;
    use lib::configuration::tls::TlsConfiguration;
    use lib::configuration::vault::VaultConfiguration;
    use lib::configuration::ServerConfiguration;
    use lib::server;
    use lib::utilities::source::Source;
    use pretty_assertions::assert_eq;
    use std::time::Duration;
    use tokio;

    extern crate lib;

    #[tokio::test]
    async fn throws_an_error_if_initialization_fails() -> Result<(), Box<dyn std::error::Error>> {
        let config = ServerConfiguration {
            health: HealthCheckConfiguration::default(),
            socket: SocketConfiguration {
                socket_path: "test_files/vault-kms-provider.sock".to_string(),
                permissions: "777".to_string(),
            },
            vault: VaultConfiguration {
                address: "https://localhost:8400".to_string(),
                transit_key: "vault-kms-provider".to_string(),
                mount_path: "transit".to_string(),
                credentials: Credentials::Token(Source::Value("invalid".to_string())),
            },
            tls: TlsConfiguration {
                cert: None,
                key: None,
                ca: None,
                directory: Some("test_files/certs".to_string()),
            },
        };
        let success = "success!".to_string();
        let result = tokio::select! {
            r = async {
                server(config).await.map(|_| "failure".to_string())
            } => r,
            r = async {
                tokio::time::sleep(Duration::from_millis(100)).await;
                Ok(success.clone())
            } => r,
        };
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn connects_and_runs_without_error() -> Result<(), Box<dyn std::error::Error>> {
        let config = ServerConfiguration {
            health: HealthCheckConfiguration::default(),
            socket: SocketConfiguration {
                socket_path: "test_files/vault-kms-provider.sock".to_string(),
                permissions: "777".to_string(),
            },
            vault: VaultConfiguration {
                address: "https://localhost:8400".to_string(),
                transit_key: "vault-kms-provider".to_string(),
                mount_path: "transit".to_string(),
                credentials: Credentials::Token(Source::Value(
                    "SiQOECxwSDCeQt1r0n5kqQCr".to_string(),
                )),
            },
            tls: TlsConfiguration {
                cert: None,
                key: None,
                ca: None,
                directory: Some("test_files/certs".to_string()),
            },
        };
        let success = "success!".to_string();
        let result = tokio::select! {
            r = async {
                server(config).await.map(|_| "failure".to_string())
            } => r,
            r = async {
                tokio::time::sleep(Duration::from_millis(100)).await;
                Ok(success.clone())
            } => r,
        }?;
        assert_eq!(result, success);
        Ok(())
    }
}
