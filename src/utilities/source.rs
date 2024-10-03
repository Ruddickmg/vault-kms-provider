use std::fs;
use vaultrs::error::ClientError;

#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    Value(String),
    FilePath(String),
}

impl Source {
    pub fn value(&self) -> Result<String, ClientError> {
        match self {
            Self::Value(value) => Ok(value.to_string()),
            Self::FilePath(path) => {
                fs::read_to_string(path).map_err(|error| ClientError::FileReadError {
                    source: error,
                    path: path.to_string(),
                })
            }
        }
    }

    pub fn path(&self) -> Option<String> {
        if let Self::FilePath(path) = self {
            Some(path.to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod source {
    use super::Source;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use std::fs;

    #[test]
    fn retrieves_a_file_from_a_file_path() {
        let path = "./test_files/source_test";
        let contents = "file contents!";
        let source = Source::FilePath(path.to_string());
        fs::write(path, contents).unwrap();
        assert_str_eq!(source.value().unwrap(), contents);
    }

    #[test]
    fn retrieves_a_value_if_defined() {
        let value = "test";
        let source = Source::Value(value.to_string());
        assert_str_eq!(source.value().unwrap(), value);
    }

    #[test]
    fn returns_a_path_if_it_is_defined() {
        let path = "./test_files/source_test";
        let source = Source::FilePath(path.to_string());
        assert_eq!(source.path(), Some(path.to_string()));
    }

    #[test]
    fn returns_none_if_no_path_is_defined() {
        let source = Source::Value("hello".to_string());
        assert_eq!(source.path(), None);
    }
}
