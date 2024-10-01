const DEFAULT_MOUNT_PATH: &str = "approle";

#[derive(Clone, Debug)]
pub struct AppRole {
    pub role_id: String,
    pub secret_id: String,
    pub mount_path: String,
}

impl AppRole {
    pub fn new(role_id: String, secret_id: String, mount_path: Option<String>) -> Self {
        Self {
            role_id,
            secret_id,
            mount_path: mount_path.unwrap_or(DEFAULT_MOUNT_PATH.to_string()),
        }
    }
}
