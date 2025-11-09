#[derive(PartialEq, Eq, Default)]
pub enum IntegrityPolicy {
  Strict,
  #[default]
  Optional,
  None,
}
