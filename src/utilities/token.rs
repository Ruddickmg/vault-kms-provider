use crate::configuration::vault::VaultConfiguration;
use std::{fs, io};
use std::io::ErrorKind;

pub fn auth_token() -> io::Result<String> {
    let config = VaultConfiguration::new();
    let token = config.vault_token;
    let path = config.vault_token_path;
    if token != "" {
        Ok(token.to_string())
    } else if path != "" {
        fs::read_to_string(path)
    } else {
        Err(std::io::Error::new(ErrorKind::Other, "No auth token found"))
    }
}
