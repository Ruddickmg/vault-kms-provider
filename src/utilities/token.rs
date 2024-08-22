use crate::configuration;
use std::{fs, io};

pub fn auth_token() -> io::Result<String> {
  let config = configuration::vault();
  let token = config.vault_token;
  let path = config.vault_token_path;
  if token != "" {
    Ok(token.to_string())
  } else if path != "" {
    fs::read_to_string(path)
  } else {
    panic!("No auth token found");
  }
}
