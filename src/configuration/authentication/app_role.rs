use crate::utilities::source::Source;

const DEFAULT_MOUNT_PATH: &str = "approle";

#[derive(Clone, Debug, PartialEq)]
pub struct AppRole {
    pub role_id: String,
    pub secret_id: Source,
    pub mount_path: String,
}

impl AppRole {
    pub fn new(role_id: String, secret_id: Source, mount_path: Option<String>) -> Self {
        Self {
            role_id,
            secret_id,
            mount_path: mount_path.unwrap_or(DEFAULT_MOUNT_PATH.to_string()),
        }
    }
}
