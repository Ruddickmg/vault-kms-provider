use crate::utilities::source::Source;

const DEFAULT_USER_PASS_AUTH_PATH: &str = "userpass";

#[derive(Clone, Debug, PartialEq)]
pub struct UserPass {
    pub username: String,
    pub password: Source,
    pub mount_path: String,
}

impl UserPass {
    pub fn new(username: String, password: Source, mount_path: Option<String>) -> Self {
        Self {
            username,
            password,
            mount_path: mount_path.unwrap_or(DEFAULT_USER_PASS_AUTH_PATH.to_string()),
        }
    }
}
