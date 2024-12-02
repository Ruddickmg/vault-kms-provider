use chrono::{DateTime, Utc};

pub fn from_iso_string_to_epoch(iso_string: &str) -> Result<i64, chrono::ParseError> {
  Ok(iso_string.parse::<DateTime<Utc>>()?.timestamp())
}

#[cfg(test)]
mod iso_date_conversion {
  use super::from_iso_string_to_epoch;

  #[test]
  fn converts_a_date_string_to_seconds_from_epoch() {
    let date = "2024-12-02T06:09:19+0000";
    assert_eq!(from_iso_string_to_epoch(date).unwrap(), 1733119759);
  }

  #[test]
  fn returns_an_error_if_the_date_cannot_be_parsed() {
    let date = "";
    assert!(from_iso_string_to_epoch(date).is_err());
  }
}
