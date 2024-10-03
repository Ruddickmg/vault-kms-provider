use crate::utilities::environment::Environment;
use tracing::Level;

const DEFAULT_LOG_LEVEL: &str = "info";
const DEFAULT_LOG_FORMAT: &str = "compact";

pub struct LoggingConfiguration {
    pub level: Level,
    pub format: String,
}

impl LoggingConfiguration {
    fn str_to_log_level(level: &str) -> Level {
        match level {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        }
    }

    pub fn new() -> Self {
        Self {
            level: Self::str_to_log_level(Environment::LogLevel.or(DEFAULT_LOG_LEVEL).as_str()),
            format: Environment::LogFormat.or(DEFAULT_LOG_FORMAT),
        }
    }
}
