use crate::configuration::authentication::Source;

const DEFAULT_KUBERNETES_AUTH_PATH: &str = "kubernetes";

#[derive(Clone, Debug)]
pub struct Kubernetes {
    pub jwt: Source,
    pub mount_path: String,
}

impl Kubernetes {
    pub fn new(source: Source, mount_path: Option<String>) -> Self {
        Self {
            jwt: source,
            mount_path: mount_path.unwrap_or(DEFAULT_KUBERNETES_AUTH_PATH.to_string()),
        }
    }
}
