use crate::configuration::vault::VaultConfiguration;
use std::io::ErrorKind;
use std::{fs, io};
use tracing::{debug, instrument};

#[instrument]
pub fn auth_token() -> io::Result<String> {
    let config = VaultConfiguration::new();
    let token = config.vault_token;
    let path = config.vault_token_path;
    if token != "" {
        debug!("Using input token of length: {}", token.len());
        Ok(token.to_string())
    } else if path != "" {
        let token = fs::read_to_string(path)?;
        debug!("Using mounted token of length: {}", token.len());
        Ok(token)
    } else {
        Err(std::io::Error::new(ErrorKind::Other, "No auth token found"))
    }
}
