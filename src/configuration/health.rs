use crate::utilities::env::get_env;

const DEFAULT_HEALTH_ENDPOINT: &str = "0.0.0.0:8080";

pub struct HealthCheckConfiguration {
  pub endpoint: String,
}

impl HealthCheckConfiguration {
  pub fn new() -> Self {
    Self {
      endpoint: get_env("HTTP_ADDRESS", DEFAULT_HEALTH_ENDPOINT),
    }
  }
}
