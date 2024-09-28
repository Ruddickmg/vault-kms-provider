use crate::configuration::logging::LoggingConfiguration;
use tracing::debug;
use tracing_subscriber::EnvFilter;

pub fn initialize() {
    let config = LoggingConfiguration::new();
    let directive = if config.level < tracing::Level::DEBUG {
        [&env!("CARGO_PKG_NAME").replace("-", "_"), "lib", "server"]
          .map(| target | format!("{}={}", target, config.level))
          .join(",")
    } else {
        config.level.to_string()
    };
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
      .with_env_filter(EnvFilter::new(&directive));

    match config.format.as_str() {
        "json" => subscriber.json().init(),
        "pretty" => subscriber.pretty().init(),
        "compact" => subscriber.compact().init(),
        _ => subscriber.init(),
    };

    debug!("Logging initialized");
}
