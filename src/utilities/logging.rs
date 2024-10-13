use crate::configuration::logging::LoggingConfiguration;
use tracing::{debug, Level};
use tracing_subscriber::EnvFilter;

pub fn initialize() {
    let config = LoggingConfiguration::default();
    let directive = if config.level < tracing::Level::DEBUG {
        [&env!("CARGO_PKG_NAME").replace("-", "_"), "lib", "server"]
            .map(|target| format!("{}={}", target, config.level))
            .join(",")
    } else {
        config.level.to_string()
    };
    let subscriber =
        tracing_subscriber::FmtSubscriber::builder().with_env_filter(EnvFilter::new(&directive));

    match config.format.as_str() {
        "json" => subscriber.json().init(),
        "pretty" => subscriber.pretty().init(),
        "compact" => subscriber.compact().init(),
        _ => subscriber.init(),
    };

    debug!("Logging initialized");
}

pub fn str_to_log_level(level: &str) -> Level {
    match level {
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => Level::INFO,
    }
}

#[cfg(test)]
mod str_to_log_level {
    use super::str_to_log_level;
    use tracing::Level;

    #[test]
    fn converts_strings_to_log_levels() {
        let converted: Vec<Level> = vec!["error", "warn", "debug", "trace", "anything", "other"]
            .iter()
            .map(|s| str_to_log_level(s))
            .collect();
        assert_eq!(
            vec![
                Level::ERROR,
                Level::WARN,
                Level::DEBUG,
                Level::TRACE,
                Level::INFO,
                Level::INFO
            ],
            converted
        );
    }
}
