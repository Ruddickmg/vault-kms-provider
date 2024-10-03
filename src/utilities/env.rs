use tracing::{debug, instrument};
use tracing_subscriber::fmt::format;
use crate::configuration::authentication::Source;

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

#[instrument(skip(var_name))]
pub fn get_env_option(var_name: &str) -> Option<String> {
    let empty = "";
    let value = get_env(var_name, empty);
    if value == empty {
        None
    } else {
        Some(value)
    }
}

#[instrument(skip(var_name))]
pub fn get_env_source_option(var_name: &str) -> Option<Source> {
    get_env_option(var_name).map(Source::Value).or(get_env_option(&format!("{}_PATH", var_name)).map(Source::FilePath))
}

#[cfg(test)]
mod test_get_env {
    use super::get_env;

    #[test]
    fn returns_an_environment_variable_if_it_exists() {
        let variable_name = "TEST_VARIABLE";
        let variable = "hello";
        unsafe {
            std::env::set_var(variable_name, variable);
        }
        let retrieved = get_env(variable_name, "");
        assert_eq!(variable, retrieved);
    }

    #[test]
    fn returns_a_default_value_if_no_variable_exists() {
        let default = "default";
        let retrieved = get_env("NON_EXISTENT_VARIABLE", default);
        assert_eq!(default, retrieved);
    }
}

#[cfg(test)]
mod test_get_env_option {
    use super::get_env_option;

    #[test]
    fn returns_an_environment_variable_if_it_exists() {
        let variable_name = "OPTION_TEST_VARIABLE_EXISTS";
        let variable = "world";
        unsafe {
            std::env::set_var(variable_name, variable);
        }
        let retrieved = get_env_option(variable_name);
        assert_eq!(Some(variable.to_string()), retrieved);
    }

    #[test]
    fn returns_none_if_the_variable_is_an_empty_string() {
        let variable_name = "OPTION_TEST_VARIABLE_EMPTY";
        let variable = "";
        unsafe {
            std::env::set_var(variable_name, variable);
        }
        let retrieved = get_env_option(variable_name);
        assert_eq!(retrieved, None);
    }

    #[test]
    fn returns_none_if_no_variable_exists() {
        let retrieved = get_env_option("NON_EXISTENT_VARIABLE");
        assert_eq!(retrieved, None);
    }
}
