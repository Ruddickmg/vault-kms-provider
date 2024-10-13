use crate::utilities::source::Source;

const DEFAULT_JWT_AUTH_MOUNT: &str = "jwt";

#[derive(Clone, Debug, PartialEq)]
pub struct Jwt {
    pub role: Option<String>,
    pub jwt: Source,
    pub mount_path: String,
}

impl Jwt {
    pub fn new(source: Source, role: Option<String>, mount_path: Option<String>) -> Self {
        Self {
            role,
            jwt: source,
            mount_path: mount_path.unwrap_or(DEFAULT_JWT_AUTH_MOUNT.to_string()),
        }
    }
}

#[cfg(test)]
mod jwt_configuration {
    use super::{Jwt, DEFAULT_JWT_AUTH_MOUNT};
    use crate::utilities::source::Source;
    use pretty_assertions::assert_eq;

    #[test]
    fn initialization_via_new_defaults_to_jwt_mount_path() {
        assert_eq!(
            Jwt::new(Source::Value("hello!".to_string()), None, None).mount_path,
            DEFAULT_JWT_AUTH_MOUNT.to_string()
        );
    }

    #[test]
    fn initialization_will_use_custom_mount_path() {
        let path = "/hello/world";
        assert_eq!(
            Jwt::new(
                Source::Value("hello!".to_string()),
                None,
                Some(path.to_string())
            )
            .mount_path,
            path.to_string()
        );
    }
}
