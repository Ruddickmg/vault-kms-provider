use crate::configuration::logging::LoggingConfiguration;
use tracing::debug;

pub fn initialize() {
    let config = LoggingConfiguration::new();
    let subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(config.level);

    match config.format.as_str() {
        "json" => subscriber.json().init(),
        "pretty" => subscriber.pretty().init(),
        "compact" => subscriber.compact().init(),
        _ => subscriber.init(),
    };

    debug!("Logging initialized");
}
