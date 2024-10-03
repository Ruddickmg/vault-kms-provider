use crate::utilities::environment::Environment;

const DEFAULT_HEALTH_ENDPOINT: &str = "0.0.0.0:8080";

pub struct HealthCheckConfiguration {
    pub endpoint: String,
}

impl HealthCheckConfiguration {
    pub fn new() -> Self {
        Self {
            endpoint: Environment::HttpAddress.or(DEFAULT_HEALTH_ENDPOINT),
        }
    }
}
