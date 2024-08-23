use tracing::Level;
use crate::utilities::env::get_env;

const DEFAULT_LOG_LEVEL: &str = "info";

pub struct LoggingConfiguration {
  pub level: String,
}

impl LoggingConfiguration {
  pub fn new() -> Self {
    Self {
      level: get_env("LOG_LEVEL", DEFAULT_LOG_LEVEL),
    }
  }
}

impl Into<tracing::Level> for LoggingConfiguration {
  fn into(self) -> Level {
    match self.level.as_str() {
      "error" => Level::ERROR,
      "warn" => Level::WARN,
      "debug" => Level::DEBUG,
      "trace" => Level::TRACE,
      _ => Level::INFO,
    }
  }
}