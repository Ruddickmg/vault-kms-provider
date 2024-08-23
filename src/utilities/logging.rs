use tracing::{Level, dispatcher::SetGlobalDefaultError};
use tracing_subscriber::FmtSubscriber;
use crate::configuration::logging::LoggingConfiguration;

pub fn initialize() -> Result<(), SetGlobalDefaultError> {
  let log_level: Level = LoggingConfiguration::new().into();
  let subscriber = FmtSubscriber::builder()
    .with_max_level(log_level)
    .finish();

  tracing::subscriber::set_global_default(subscriber)
}
