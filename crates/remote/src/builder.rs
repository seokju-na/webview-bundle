use crate::Remote;

pub trait Builder: Default + 'static {
  type Config: crate::Config;

  fn build(self) -> crate::Result<impl Remote>;
}
