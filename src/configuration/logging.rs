use crate::utilities::environment::Environment;
use crate::utilities::logging::str_to_log_level;
use tracing::Level;

const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_LOG_FORMAT: &str = "compact";

#[derive(Debug, Clone, PartialEq)]
pub struct LoggingConfiguration {
    pub level: Level,
    pub format: String,
}

impl Default for LoggingConfiguration {
    fn default() -> Self {
        Self {
            level: str_to_log_level(Environment::LogLevel.or(DEFAULT_LOG_LEVEL).as_str()),
            format: Environment::LogFormat.or(DEFAULT_LOG_FORMAT),
        }
    }
}

#[cfg(test)]
mod logging_configuration {
    use super::{LoggingConfiguration, DEFAULT_LOG_FORMAT};
    use tracing::Level;

    #[test]
    fn initializes_values_with_default() {
        assert_eq!(
            LoggingConfiguration::default(),
            LoggingConfiguration {
                level: Level::INFO,
                format: DEFAULT_LOG_FORMAT.to_string(),
            }
        )
    }
}
