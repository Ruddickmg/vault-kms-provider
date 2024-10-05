use crate::utilities::source::Source;

const DEFAULT_KUBERNETES_AUTH_MOUNT: &str = "kubernetes";
const DEFAULT_VAULT_ROLE: &str = "vault-kms-provider";

#[derive(Clone, Debug)]
pub struct Kubernetes {
    pub role: String,
    pub jwt: Source,
    pub mount_path: String,
}

impl Kubernetes {
    pub fn new(source: Source, role: Option<String>, mount_path: Option<String>) -> Self {
        Self {
            role: role.unwrap_or(DEFAULT_VAULT_ROLE.to_string()),
            jwt: source,
            mount_path: mount_path.unwrap_or(DEFAULT_KUBERNETES_AUTH_MOUNT.to_string()),
        }
    }
}

#[cfg(test)]
mod jwt_configuration {
    use super::{Kubernetes, DEFAULT_KUBERNETES_AUTH_MOUNT};
    use crate::utilities::source::Source;
    use pretty_assertions::assert_eq;

    #[test]
    fn initialization_defaults_to_kubernetes_mount_path() {
        assert_eq!(
            Kubernetes::kubernetes(Source::Value("hello!".to_string()), None).mount_path,
            DEFAULT_KUBERNETES_AUTH_MOUNT.to_string()
        );
    }

    #[test]
    fn initialization_will_use_custom_mount_path() {
        let path = "/hello/world";
        assert_eq!(
            Kubernetes::kubernetes(Source::Value("hello!".to_string()), Some(path.to_string()))
                .mount_path,
            path.to_string()
        );
    }
}
