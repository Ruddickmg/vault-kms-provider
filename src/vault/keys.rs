use std::collections::HashMap;

pub struct KeyInfo {
    pub id: String,
    pub version: String,
}

impl From<&HashMap<String, u64>> for KeyInfo {
    fn from(value: &HashMap<String, u64>) -> Self {
        let mut keys: Vec<(String, String)> = value
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<(String, String)>>();
        keys.sort_by(|(a, _), (b, _)| a.cmp(b));
        let (version, id) = keys.first().unwrap();
        KeyInfo {
            version: version.to_string(),
            id: id.to_string(),
        }
    }
}

impl From<HashMap<String, u64>> for KeyInfo {
    fn from(value: HashMap<String, u64>) -> Self {
        KeyInfo::from(&value)
    }
}
