pub trait Config: 'static {
  type Builder: crate::Builder;
  fn into_builder(self) -> Self::Builder;
}
