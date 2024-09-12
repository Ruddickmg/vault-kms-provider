use tracing::{debug, instrument};

#[instrument(skip(var_name, default))]
pub fn get_env(var_name: &str, default: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| {
        debug!(
            "{}",
            format!(
                "Environment variable: \"{}\" has not been defined, defaulting to: {}",
                var_name, default
            )
        );
        default.to_string()
    })
}
