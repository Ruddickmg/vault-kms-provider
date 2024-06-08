use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data<'a, T: Deserialize<'a>> {
  pub data: T
}

impl<'a, T: Deserialize<'a>> Data<'a, T> {
  pub fn new(data: T) -> Data<'a, T> {
    Data {
      data
    }
  }
}