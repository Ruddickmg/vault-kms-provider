const DEFAULT_KUBERNETES_AUTH_PATH: &str = "kubernetes";

#[derive(Clone, Debug)]
pub struct Kubernetes {
  pub file_path: String,
  pub mount_path: String,
}

impl Kubernetes {
  pub fn new(file_path: String, mount_path: Option<String>) -> Self {
    Self {
      file_path,
      mount_path: mount_path.unwrap_or(DEFAULT_KUBERNETES_AUTH_PATH.to_string()),
    }
  }
}