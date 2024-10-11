use crate::utilities::environment::Environment;

const DEFAULT_HEALTH_ENDPOINT: &str = "0.0.0.0:8080";

pub struct HealthCheckConfiguration {
    pub endpoint: String,
}

impl Default for HealthCheckConfiguration {
    fn default() -> Self {
        Self {
            endpoint: Environment::HttpAddress.or(DEFAULT_HEALTH_ENDPOINT),
        }
    }
}
