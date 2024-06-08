use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data<T> {
  pub data: T
}
