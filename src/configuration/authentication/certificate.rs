const DEFAULT_MOUNT_PATH: &str = "cert";

#[derive(Clone, Debug)]
pub struct Certificate {
    pub name: String,
    pub mount_path: String,
}

impl Certificate {
    pub fn new(name: String, mount_path: Option<String>) -> Self {
        Self {
            name,
            mount_path: mount_path.unwrap_or(DEFAULT_MOUNT_PATH.to_string()),
        }
    }
}
