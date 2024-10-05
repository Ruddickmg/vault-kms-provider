use crate::utilities::source::Source;
use convert_case::{Case, Casing};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tracing::{debug, instrument};

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Environment {
    VaultCertificateName,
    VaultJwt,
    VaultJwtPath,
    VaultJwtRole,
    VaultAuthMount,
    VaultToken,
    VaultTokenPath,
    VaultKubernetesJwt,
    VaultKubernetesJwtPath,
    VaultKubernetesRole,
    VaultPassword,
    VaultPasswordPath,
    VaultUser,
    VaultRoleId,
    VaultSecretId,
    VaultSecretIdPath,
    HttpAddress,
    LogLevel,
    LogFormat,
    SocketPath,
    SocketPermissions,
    VaultCaPath,
    VaultCaCert,
    VaultClientCert,
    VaultClientKey,
    VaultAddress,
    VaultTransitKey,
    VaultTransitMount,
    Unknown,
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self).to_case(Case::Snake).to_uppercase())
    }
}

impl From<&str> for Environment {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for Environment {
    fn from(value: String) -> Self {
        Environment::iter()
            .find(|env| env.to_string() == value)
            .unwrap_or(Environment::Unknown)
    }
}

impl Environment {
    #[instrument]
    pub fn get(&self) -> Option<String> {
        let value = std::env::var(self.to_string()).ok();
        if self == &Self::Unknown {
            None
        } else if let Some(val) = value {
            if &val == "" {
                None
            } else {
                Some(val)
            }
        } else {
            debug!(
                "{}",
                format!(
                    "Environment variable: \"{}\" has not been defined",
                    self.to_string(),
                )
            );
            None
        }
    }

    #[instrument]
    pub fn or(&self, default: &str) -> String {
        let value = self.get();
        if value.is_none() {
            debug!(
                "Defaulting environment variable \"{}\" to: \"{}\"",
                self.to_string(),
                default
            );
        }
        value.unwrap_or(default.to_string())
    }

    #[instrument]
    pub fn source(&self) -> Option<Source> {
        if self == &Self::Unknown {
            None
        } else {
            self.get().map(Source::Value).or(Environment::from(format!(
                "{}_PATH",
                &self.to_string()
            ))
            .get()
            .map(Source::FilePath))
        }
    }
}

#[cfg(test)]
mod environment {
    use super::Environment;
    use crate::utilities::source::Source;
    use pretty_assertions::{assert_eq, assert_str_eq};

    #[test]
    fn or_returns_an_environment_variable_if_it_exists() {
        let env_var = Environment::VaultTransitKey;
        let variable = "hello";
        unsafe {
            std::env::set_var(env_var.to_string(), variable);
        }
        let retrieved = env_var.or("");
        assert_eq!(variable, retrieved);
    }

    #[test]
    fn or_returns_a_default_value_if_no_variable_exists() {
        let default = "default";
        let retrieved = Environment::HttpAddress.or(default);
        assert_eq!(default, retrieved);
    }

    #[test]
    fn get_returns_an_environment_variable_if_it_exists() {
        let env_var = Environment::VaultSecretIdPath;
        let variable = "world";
        unsafe {
            std::env::set_var(env_var.to_string(), variable);
        }
        let retrieved = env_var.get();
        assert_eq!(Some(variable.to_string()), retrieved);
    }

    #[test]
    fn get_returns_none_if_the_variable_is_an_empty_string() {
        let env_var = Environment::VaultTokenPath;
        let variable = "";
        unsafe {
            std::env::set_var(env_var.to_string(), variable);
        }
        let retrieved = env_var.get();
        assert_eq!(retrieved, None);
    }

    #[test]
    fn get_returns_none_if_no_variable_exists() {
        assert_eq!(Environment::VaultPasswordPath.get(), None);
    }

    #[test]
    fn source_returns_a_file_path_if_no_value_exists() {
        let env_var = Environment::VaultPassword;
        let path = "./some/file/path";
        unsafe {
            std::env::set_var(&format!("{}_PATH", env_var.to_string()), path);
        }
        assert_eq!(env_var.source(), Some(Source::FilePath(path.to_string())))
    }

    #[test]
    fn source_returns_a_value_if_one_exists() {
        let env_var = Environment::VaultAddress;
        let value = "http://127.0.0.1:8200";
        unsafe {
            std::env::set_var(env_var.to_string(), value);
        }
        assert_eq!(env_var.source(), Some(Source::Value(value.to_string())))
    }

    #[test]
    fn source_returns_none_if_no_value_or_file_path_exists() {
        assert_eq!(Environment::VaultAuthMount.source(), None)
    }

    #[test]
    fn converts_enum_name_to_environment_variable_name() {
        assert_str_eq!(
            Environment::VaultPasswordPath.to_string(),
            "VAULT_PASSWORD_PATH".to_string()
        );
    }

    #[test]
    fn get_returns_none_for_unknown_environment_variables() {
        unsafe {
            std::env::set_var(Environment::Unknown.to_string(), "something!");
        }
        assert_eq!(Environment::Unknown.get(), None);
    }

    #[test]
    fn or_returns_default_value_for_unknown_environment_variables() {
        let default = "default!";
        unsafe {
            std::env::set_var(Environment::Unknown.to_string(), "something!");
        }
        assert_eq!(Environment::Unknown.or(default), default.to_string());
    }

    #[test]
    fn source_returns_none_for_unknown_environment_variables() {
        unsafe {
            std::env::set_var(Environment::Unknown.to_string(), "something!");
        }
        assert_eq!(Environment::Unknown.source(), None);
    }
}
