use std::fmt::Display;

pub enum TransitPath {
  Encrypt,
  Decrypt,
  Keys,
}

impl Display for TransitPath {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      TransitPath::Encrypt => "encrypt".to_string(),
      TransitPath::Decrypt => "decrypt".to_string(),
      TransitPath::Keys => "keys".to_string(),
    };
    write!(f, "{}", str)
  }
}