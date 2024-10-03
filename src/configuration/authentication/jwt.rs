use crate::utilities::source::Source;

const DEFAULT_KUBERNETES_AUTH_PATH: &str = "kubernetes";
const DEFAULT_JWT_AUTH_PATH: &str = "jwt";

#[derive(Clone, Debug)]
pub struct Jwt {
    pub jwt: Source,
    pub mount_path: String,
}

impl Jwt {
    pub fn kubernetes(source: Source, mount_path: Option<String>) -> Self {
        Self::new(source, mount_path.or(Some(DEFAULT_KUBERNETES_AUTH_PATH.to_string())))
    }

    pub fn new(source: Source, mount_path: Option<String>) -> Self {
        Self {
            jwt: source,
            mount_path: mount_path.unwrap_or(DEFAULT_JWT_AUTH_PATH.to_string()),
        }
    }
}


#[cfg(test)]
mod jwt_configuration {
    use pretty_assertions::assert_eq;
    use crate::utilities::source::Source;
    use super::{DEFAULT_JWT_AUTH_PATH, DEFAULT_KUBERNETES_AUTH_PATH, Jwt};

    #[test]
    fn kubernetes_initialization_defaults_to_kubernetes_mount_path() {
        assert_eq!(Jwt::kubernetes(Source::Value("hello!".to_string()), None).mount_path, DEFAULT_KUBERNETES_AUTH_PATH.to_string());
    }

    #[test]
    fn kubernetes_initialization_will_use_custom_mount_path() {
        let path = "/hello/world";
        assert_eq!(Jwt::kubernetes(Source::Value("hello!".to_string()), Some(path.to_string())).mount_path, path.to_string());
    }

    #[test]
    fn initialization_via_new_defaults_to_jwt_mount_path() {
        assert_eq!(Jwt::new(Source::Value("hello!".to_string()), None).mount_path, DEFAULT_JWT_AUTH_PATH.to_string());
    }

    #[test]
    fn initialization_will_use_custom_mount_path() {
        let path = "/hello/world";
        assert_eq!(Jwt::new(Source::Value("hello!".to_string()), Some(path.to_string())).mount_path, path.to_string());
    }
}