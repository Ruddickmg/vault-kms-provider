use std::collections::HashMap;
use vaultrs::api::transit::responses::ReadKeyData;

#[derive(Clone, Debug, PartialEq)]
pub struct KeyInfo {
    pub id: String,
    pub version: String,
}

impl From<&ReadKeyData> for KeyInfo {
    fn from(value: &ReadKeyData) -> Self {
        match value {
            ReadKeyData::Asymmetric(_data) => {
                // TODO: uncomment after adding configurations for asymmetric key usage
                // data.into()
                panic!("Asymmetric keys not yet supported");
            }
            ReadKeyData::Symmetric(data) => data.into(),
        }
    }
}

impl From<&HashMap<String, u64>> for KeyInfo {
    fn from(value: &HashMap<String, u64>) -> Self {
        let mut keys: Vec<(String, String)> = value
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect::<Vec<(String, String)>>();
        keys.sort_by(|(_, a), (_, b)| b.cmp(a));
        let (version, id) = keys.first().unwrap();
        KeyInfo {
            version: version.to_string(),
            id: id.to_string(),
        }
    }
}
// TODO: uncomment after adding configurations for asymmetric key usage
// impl From<&HashMap<String, ReadPublicKeyEntry>> for KeyInfo {
//     fn from(value: &HashMap<String, ReadPublicKeyEntry>) -> Self {
//         let mut keys: Vec<(String, String)> = value
//             .iter()
//             .map(|(a, b)| {
//                 (
//                     a.to_string(),
//                     from_iso_string_to_epoch(&b.creation_time)
//                         .unwrap()
//                         .to_string(),
//                 )
//             })
//             .collect::<Vec<(String, String)>>();
//         keys.sort_by(|(_, a), (_, b)| b.cmp(a));
//         let (version, id) = keys.first().unwrap();
//         KeyInfo {
//             version: version.to_string(),
//             id: id.to_string(),
//         }
//     }
// }

impl From<HashMap<String, u64>> for KeyInfo {
    fn from(value: HashMap<String, u64>) -> Self {
        KeyInfo::from(&value)
    }
}

// TODO: uncomment after adding configurations for asymmetric key usage
// impl From<HashMap<String, ReadPublicKeyEntry>> for KeyInfo {
//     fn from(value: HashMap<String, ReadPublicKeyEntry>) -> Self {
//         KeyInfo::from(&value)
//     }
// }

impl From<ReadKeyData> for KeyInfo {
    fn from(value: ReadKeyData) -> Self {
        KeyInfo::from(&value)
    }
}

#[cfg(test)]
mod key_info {
    use super::KeyInfo;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;
    use std::time::{SystemTime, UNIX_EPOCH};
    use vaultrs::api::transit::responses::{ReadKeyData, ReadPublicKeyEntry};

    #[test]
    #[should_panic]
    fn panics_when_an_asymmetric_key_is_encountered() {
        let data: HashMap<String, ReadPublicKeyEntry> = HashMap::new();
        let key_data: ReadKeyData = ReadKeyData::Asymmetric(data);
        let _ = KeyInfo::from(key_data);
    }

    #[test]
    fn gets_most_recent_key_from_hash_map_of_keys() {
        let mut map: HashMap<String, u64> = HashMap::new();
        let start = SystemTime::now();
        let mut since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut latest_id = String::new();
        for n in 1..10 {
            since_the_epoch += 1;
            map.insert(format!("{}", n), since_the_epoch);
            latest_id = format!("{}", since_the_epoch);
        }
        assert_eq!(
            KeyInfo::from(map),
            KeyInfo {
                id: latest_id,
                version: "9".to_string()
            }
        );
    }
}
