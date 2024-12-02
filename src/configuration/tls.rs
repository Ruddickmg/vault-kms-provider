use crate::utilities::environment::Environment;
use std::fs;
use tracing::info;

#[derive(Clone, Debug, PartialEq)]
pub struct TlsConfiguration {
    pub cert: Option<String>,
    pub key: Option<String>,
    pub ca: Option<String>,
    pub directory: Option<String>,
}

impl Default for TlsConfiguration {
    fn default() -> Self {
        Self {
            directory: Environment::VaultCaPath.get(),
            cert: Environment::VaultClientCert.get(),
            key: Environment::VaultClientKey.get(),
            ca: Environment::VaultCaCert.get(),
        }
    }
}

impl TlsConfiguration {
    pub fn identity(&self) -> Option<vaultrs::client::Identity> {
        if let Some((key, cert)) = self.cert.clone().zip(self.key.clone()) {
            if let Some(pem) =
                fs::read(cert)
                    .ok()
                    .zip(fs::read(key).ok())
                    .map(|(mut raw_cert, mut raw_key)| {
                        raw_cert.append(&mut raw_key);
                        raw_cert
                    })
            {
                vaultrs::client::Identity::from_pem(pem.as_slice()).ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn certs(&self) -> Vec<String> {
        let mut certs = self.certs_from_dir();
        if let Some(file_path) = self.ca.clone() {
            if !certs.contains(&file_path) {
                certs.push(file_path);
            }
        }
        info!("Importing CA certificates: {:?}", certs);
        certs
    }

    fn certs_from_dir(&self) -> Vec<String> {
        if let Some(path) = self.directory.clone() {
            if let Ok(paths) = fs::read_dir(&path) {
                let a: Vec<String> = paths
                    .filter(|p| p.is_ok())
                    .map(|p| p.unwrap().path())
                    .filter(|p| p.is_file())
                    .map(|p| p.to_str().unwrap().to_string())
                    .filter(|p| p.as_str() != "")
                    .collect();
                return a;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tls_certificate_tests {
    use super::*;
    use std::collections::HashSet;
    use std::path::Path;

    mod certs {
        use super::*;

        #[test]
        pub fn gets_all_files_located_in_a_directory_path() -> () {
            let mut config = TlsConfiguration::default();
            config.directory = Some("./test_files".to_string());
            let certs = config.certs();
            let filtered: Vec<&String> = certs
                .iter()
                .filter(|o| Path::new(&o.to_string()).is_file())
                .collect();
            assert_ne!(filtered.len(), 0);
            assert_eq!(filtered.len(), certs.len());
        }

        #[test]
        pub fn gets_a_specified_cert_file_path() -> () {
            let path = "./test_files/certs/ca.crt".to_string();
            let config = TlsConfiguration {
                ca: Some(path.clone()),
                directory: None,
                cert: None,
                key: None,
            };
            assert_eq!(config.certs(), vec![path])
        }

        #[test]
        pub fn will_not_create_duplicate_file_paths() -> () {
            let config = TlsConfiguration {
                directory: Some("./test_files/certs".to_string()),
                ca: Some("./test_files/certs/ca.crt".to_string()),
                cert: None,
                key: None,
            };
            let mut certs = config.certs();
            let mut unique: Vec<String> = certs
                .clone()
                .into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            assert_eq!(certs.sort(), unique.sort());
        }
    }

    mod identity {
        use super::*;

        #[test]
        fn returns_identity_if_both_cert_and_key_are_valid() {
            let config = TlsConfiguration {
                directory: Some("./test_files/certs".to_string()),
                ca: Some("./test_files/certs/ca.crt".to_string()),
                cert: Some("./test_files/certs/tls.crt".to_string()),
                key: Some("./test_files/certs/tls.key".to_string()),
            };
            assert!(config.identity().is_some());
        }

        #[test]
        fn returns_none_if_key_is_missing() {
            let config = TlsConfiguration {
                directory: Some("./test_files/certs".to_string()),
                ca: Some("./test_files/certs/ca.crt".to_string()),
                cert: Some("./test_files/certs/tls.crt".to_string()),
                key: None,
            };
            assert!(config.identity().is_none());
        }

        #[test]
        fn returns_none_if_both_cert_is_missing() {
            let config = TlsConfiguration {
                directory: Some("./test_files/certs".to_string()),
                ca: Some("./test_files/certs/ca.crt".to_string()),
                cert: None,
                key: Some("./test_files/certs/tls.key".to_string()),
            };
            assert!(config.identity().is_none());
        }

        #[test]
        fn returns_none_if_cert_or_key_are_invalid() {
            let config = TlsConfiguration {
                directory: Some("./test_files/certs".to_string()),
                ca: Some("./test_files/certs/ca.crt".to_string()),
                cert: Some("./test_files/certs/tls.crt".to_string()),
                key: Some("./test_files/values.yaml".to_string()),
            };
            assert!(config.identity().is_none());
        }
    }
}
