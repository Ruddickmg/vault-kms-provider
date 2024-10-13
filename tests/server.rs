mod common;

#[cfg(test)]
mod server {
    use super::common;
    use lib::configuration::authentication::Credentials;
    use lib::server;
    use lib::utilities::logging;
    use lib::utilities::source::Source;
    use pretty_assertions::assert_eq;
    use std::time::Duration;
    use tokio;

    extern crate lib;

    #[tokio::test]
    async fn throws_an_error_if_initialization_fails() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = common::server_config();
        config.vault.credentials = Credentials::Token(Source::Value("invalid".to_string()));
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
        logging::initialize();
        let config = common::server_config();
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
