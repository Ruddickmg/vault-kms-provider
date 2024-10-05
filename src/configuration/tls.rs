use crate::utilities::environment::Environment;
use std::fs;
use tracing::{debug, info};

pub struct TlsConfiguration {
    pub cert: Option<String>,
    pub key: Option<String>,
    pub ca: Option<String>,
    pub directory: Option<String>,
}

impl TlsConfiguration {
    pub fn new() -> Self {
        Self {
            directory: Environment::VaultCaPath.get(),
            cert: Environment::VaultClientCert.get(),
            key: Environment::VaultClientKey.get(),
            ca: Environment::VaultCaCert.get(),
        }
    }

    pub fn identity(&self) -> Option<reqwest::Identity> {
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
                reqwest::Identity::from_pem(pem.as_slice()).ok()
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
        certs.iter().for_each(|cert| {
            debug!("cert: \n{}", fs::read_to_string(cert).unwrap());
        });
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

// #[test]
// mod tls_certificate_tests {
//     use super::*;
//
//     #[test]
//     pub fn gets_all_certs_located_in_a_directory_path() -> () {}
//
//     #[test]
//     pub fn gets_a_specified_cert_file_path() -> () {}
//
//     #[test]
//     pub fn will_not_create_duplicate_file_paths() -> () {}
//
//     #[test]
//     pub fn will_combine_files_from_directory_with_specific_files_if_both_are_defined() -> () {}
// }
