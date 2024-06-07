use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data<T: Deserialize> {
  pub data: T
}

impl<T: Deserialize> Data<T> {
  pub fn new(data: T) -> Self<T> {
    Data {
      data
    }
  }
}