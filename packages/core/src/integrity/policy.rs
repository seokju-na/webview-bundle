#[derive(PartialEq, Eq)]
pub enum IntegrityPolicy {
  Strict,
  Optional,
  None,
}

impl Default for IntegrityPolicy {
  fn default() -> Self {
    Self::Optional
  }
}
