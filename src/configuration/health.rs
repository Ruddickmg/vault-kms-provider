use crate::utilities::environment::Environment;

const DEFAULT_HEALTH_ENDPOINT: &str = "0.0.0.0:8080";

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod health_configuration {
    use super::{HealthCheckConfiguration, DEFAULT_HEALTH_ENDPOINT};
    use pretty_assertions::assert_eq;

    #[test]
    fn initializes_health_check_endpoint_with_default() {
        assert_eq!(
            HealthCheckConfiguration::default(),
            HealthCheckConfiguration {
                endpoint: DEFAULT_HEALTH_ENDPOINT.to_string(),
            }
        )
    }
}
